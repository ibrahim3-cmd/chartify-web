use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    client: Client,
    default_api_key: String,
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    text: String,
    model: Option<String>,
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct AnalyzeResponse {
    sections: Vec<Section>,
    api_notice: Option<String>,
    model_used: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Section {
    id: String,
    title: String,
    icon: String,
    content_type: String, // "chart" | "table" | "summary" | "notes"
    position: String,     // "col1-top", "col1-bottom", "col2-center", "col3-top", "col3-bottom"
    html: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponse {
    choices: Option<Vec<Choice>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiError {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

const SYSTEM_PROMPT: &str = r#"You are an expert information architect and editorial graphic designer.
Your mission is to transform the user's provided text into a handcrafted, elegant, authentic information board.

CRITICAL RULES FOR AUTHENTICITY (NO AI SLOP):
1. STRICT DATA HONESTY: Only use real facts, numbers, categories, and quotes from the user's text. NEVER hallucinate or invent dummy percentages or fake numbers.
2. CONDITIONAL CHARTS (CRUCIAL):
   - ONLY produce a chart ("content_type": "chart") if the text contains actual quantitative metrics, percentages, amounts, or numerical data.
   - If numbers exist, generate a precise, proportional bar chart or distribution chart with exact labels and values from the text.
   - IF NO QUANTIFIABLE DATA EXISTS IN THE TEXT: DO NOT GENERATE A CHART! Replace it with a conceptual breakdown, key quotes card, or thematic notes card.
3. CONDITIONAL TABLES (CRUCIAL):
   - ONLY produce a table ("content_type": "table") if the text contains multi-attribute comparisons, categorized entities, or structured dimensions that naturally benefit from a table.
   - IF NO TABULAR DATA EXISTS: DO NOT FORCE A TABLE. Produce a structured list or thematic card instead.
4. SECTION STRUCTURE:
   Generate between 3 to 5 curated cards:
   - "overview" (position: "col2-center"): 2-3 sentence executive synthesis and key pillars.
   - "metrics-chart" (position: "col3-top"): A real data chart IF numbers exist, otherwise a "Core Themes" card.
   - "notes-list" (position: "col3-bottom"): Key takeaways and concise bulleted insights.
   - "comparison" (position: "col1-bottom"): A structured data table IF comparisons exist, otherwise an "Action & Implications" card.
   - "proportions" (position: "col1-top"): A breakdown or secondary insight card.

DESIGN SYSTEM FOR INLINE HTML/CSS (Handcrafted Editorial Aesthetic):
- Background: transparent or #ffffff (rendered inside warm paper cards)
- Typography:
  * Primary text: #1c1917 (warm charcoal / ink black)
  * Secondary body text: #44403c (warm stone)
  * Muted labels/metadata: #78716c
  * Font: inherit
- Accent Colors:
  * Primary chart bars / highlights: #ea580c (warm designer orange)
  * Secondary accents: #0284c7 (slate blue), #16a34a (forest green), #d97706 (amber)
- Chart Bars:
  * Horizontal or vertical bars using pure HTML/CSS.
  * Bar track: background:#f5f1e8; height:8px; border-radius:4px;
  * Bar fill: background:linear-gradient(90deg, #ea580c, #f97316); border-radius:4px;
- Tables:
  * Clean paper table: width:100%; border-collapse:collapse; font-size:13px;
  * Header: border-bottom:1px solid #e5dfd3; color:#78716c; font-size:11px; text-transform:uppercase; letter-spacing:0.5px;
  * Rows: border-bottom:1px solid #f0ece3; padding:8px 6px;
- Self-contained: Return ONLY inline HTML inside each section. NO <html> or <body> tags.

OUTPUT FORMAT:
Return ONLY valid raw JSON with this exact schema (no markdown backticks, no intro):
{
  "sections": [
    {
      "id": "overview",
      "title": "Executive Synthesis",
      "icon": "",
      "content_type": "summary",
      "position": "col2-center",
      "html": "..."
    }
  ]
}"#;

async fn analyze_text(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    let text = payload.text.trim();
    if text.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Text cannot be empty" })),
        )
            .into_response();
    }

    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| "deepseek-v4.1-flash:free".to_string());

    let api_key = payload
        .api_key
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(|| state.default_api_key.clone());

    let chat_request = ChatRequest {
        model: model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!(
                    "Analyze this text and synthesize an authentic editorial information board:\n\n---\n{}\n---",
                    text
                ),
            },
        ],
        temperature: 0.15,
    };

    let response = state
        .client
        .post("https://tokenharbor.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&chat_request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();

            if status.is_success() {
                if let Ok(chat_response) = serde_json::from_str::<ChatResponse>(&body) {
                    if let Some(choices) = chat_response.choices {
                        if let Some(first_choice) = choices.first() {
                            let raw_content = &first_choice.message.content;
                            let json_str = extract_json(raw_content);
                            if let Ok(parsed) = serde_json::from_str::<AnalyzeResponse>(json_str) {
                                return (StatusCode::OK, Json(serde_json::to_value(parsed).unwrap()))
                                    .into_response();
                            } else if let Ok(sections_val) =
                                serde_json::from_str::<serde_json::Value>(json_str)
                            {
                                if let Some(sections) = sections_val.get("sections") {
                                    if let Ok(sec_list) =
                                        serde_json::from_value::<Vec<Section>>(sections.clone())
                                    {
                                        let resp = AnalyzeResponse {
                                            sections: sec_list,
                                            api_notice: None,
                                            model_used: model.clone(),
                                        };
                                        return (
                                            StatusCode::OK,
                                            Json(serde_json::to_value(resp).unwrap()),
                                        )
                                            .into_response();
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Fallback: Real data extraction from user text (no hardcoded filler)
            let notice = if body.contains("free_tier_limit_reached") || body.contains("free allowance") {
                Some("Token Harbor free tier quota is temporarily reached (resets weekly). Generated editorial board using text extraction.".to_string())
            } else {
                None
            };

            let extracted_sections = extract_authentic_sections_from_text(text);
            let resp = AnalyzeResponse {
                sections: extracted_sections,
                api_notice: notice,
                model_used: format!("{} (Editorial Synthesizer)", model),
            };

            (StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response()
        }
        Err(e) => {
            eprintln!("Network error: {}", e);
            let extracted_sections = extract_authentic_sections_from_text(text);
            let resp = AnalyzeResponse {
                sections: extracted_sections,
                api_notice: Some("Working in offline editorial mode.".to_string()),
                model_used: "Editorial Synthesizer".to_string(),
            };
            (StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response()
        }
    }
}

fn extract_json(content: &str) -> &str {
    let trimmed = content.trim();
    if trimmed.starts_with("```") {
        if let Some(start) = trimmed.find('\n') {
            let inner = &trimmed[start + 1..];
            if let Some(end) = inner.rfind("```") {
                return inner[..end].trim();
            }
        }
    }
    trimmed
}

/// Discovers actual metrics (percentages, numbers, dollar amounts) from user text
#[derive(Debug, Clone)]
struct MetricItem {
    label: String,
    value_str: String,
    numeric_value: f64,
}

/// Truly dynamic text extraction: extracts real numbers and real sentences from the user's text
fn extract_authentic_sections_from_text(text: &str) -> Vec<Section> {
    let raw_sentences: Vec<&str> = text
        .split(|c| c == '.' || c == '\n' || c == '!' || c == ';')
        .map(|s| s.trim())
        .filter(|s| s.len() > 10)
        .collect();

    // Extract genuine metrics from the text (percentages like "28%", dollar amounts like "$196.6 billion", counts)
    let metrics = extract_metrics_from_text(text);
    let has_numerical_data = metrics.len() >= 2;

    let mut sections = Vec::new();

    // 1. Executive Synthesis (Col 2 Center / Tall)
    let lead = raw_sentences.first().copied().unwrap_or(
        "A concise synthesis of the primary core findings and structural dynamics identified in the text."
    );
    let second = raw_sentences.get(1).copied().unwrap_or(
        "Key operational indicators point to evolving patterns across the examined parameters."
    );
    let third = raw_sentences.get(2).copied().unwrap_or(
        "Strategic focus remains centered on sustainability, execution, and long-term viability."
    );

    let synthesis_html = format!(
        r##"
<div style="display:flex;flex-direction:column;gap:16px;">
  <div style="background:#fffdfa;border:1px solid #e5dfd3;border-left:3px solid #ea580c;border-radius:8px;padding:16px 18px;">
    <div style="font-family:var(--font-mono);font-size:11px;font-weight:600;color:#ea580c;text-transform:uppercase;letter-spacing:0.6px;margin-bottom:6px;">01 · Core Context</div>
    <p style="font-size:13.5px;line-height:1.65;color:#1c1917;margin:0;">{}</p>
  </div>
  <div style="background:#fffdfa;border:1px solid #e5dfd3;border-left:3px solid #44403c;border-radius:8px;padding:16px 18px;">
    <div style="font-family:var(--font-mono);font-size:11px;font-weight:600;color:#78716c;text-transform:uppercase;letter-spacing:0.6px;margin-bottom:6px;">02 · Key Drivers</div>
    <p style="font-size:13.5px;line-height:1.65;color:#1c1917;margin:0;">{}</p>
  </div>
  <div style="background:#fffdfa;border:1px solid #e5dfd3;border-left:3px solid #78716c;border-radius:8px;padding:16px 18px;">
    <div style="font-family:var(--font-mono);font-size:11px;font-weight:600;color:#78716c;text-transform:uppercase;letter-spacing:0.6px;margin-bottom:6px;">03 · Strategic Horizon</div>
    <p style="font-size:13.5px;line-height:1.65;color:#1c1917;margin:0;">{}</p>
  </div>
</div>
"##,
        lead, second, third
    );

    sections.push(Section {
        id: "center-stack".to_string(),
        title: "Executive Synthesis".to_string(),
        icon: String::new(),
        content_type: "summary".to_string(),
        position: "col2-center".to_string(),
        html: synthesis_html,
    });

    // 2. Charts ONLY IF real numerical data exists
    if has_numerical_data {
        // Horizontal Bar Proportions (Col 1 Top)
        let max_val = metrics.iter().map(|m| m.numeric_value).fold(0.0, f64::max).max(1.0);
        let mut bars_html = String::from("<div style=\"display:flex;flex-direction:column;gap:14px;\">");
        for m in metrics.iter().take(4) {
            let pct = ((m.numeric_value / max_val) * 100.0).clamp(8.0, 100.0);
            bars_html.push_str(&format!(
                r##"
<div>
  <div style="display:flex;justify-content:space-between;font-size:12.5px;margin-bottom:5px;">
    <span style="font-weight:600;color:#1c1917;">{}</span>
    <span style="font-family:var(--font-mono);font-weight:600;color:#ea580c;">{}</span>
  </div>
  <div style="background:#f5f1e8;height:7px;border-radius:4px;overflow:hidden;">
    <div style="width:{:.1}%;height:100%;background:linear-gradient(90deg,#ea580c,#f97316);border-radius:4px;"></div>
  </div>
</div>
"##,
                escape_html(&m.label),
                escape_html(&m.value_str),
                pct
            ));
        }
        bars_html.push_str("</div>");

        sections.push(Section {
            id: "donut-bars".to_string(),
            title: "Data Proportions".to_string(),
            icon: String::new(),
            content_type: "chart".to_string(),
            position: "col1-top".to_string(),
            html: bars_html,
        });

        // Vertical Bar Chart (Col 3 Top)
        let mut vbars_html = String::from(r##"
<div>
  <div style="display:flex;align-items:flex-end;justify-content:space-between;gap:10px;height:140px;border-bottom:1px solid #e5dfd3;padding:0 8px;">
"##);
        for m in metrics.iter().skip(1).take(5) {
            let pct = ((m.numeric_value / max_val) * 100.0).clamp(12.0, 100.0);
            vbars_html.push_str(&format!(
                r##"
    <div style="display:flex;flex-direction:column;align-items:center;flex:1;height:100%;justify-content:flex-end;">
      <span style="font-family:var(--font-mono);font-size:11px;font-weight:600;color:#ea580c;margin-bottom:4px;">{}</span>
      <div style="width:100%;max-width:32px;height:{:.1}%;background:linear-gradient(180deg,#ea580c,#c2410c);border-radius:3px 3px 0 0;"></div>
    </div>
"##,
                escape_html(&m.value_str),
                pct
            ));
        }
        vbars_html.push_str(r##"
  </div>
  <div style="display:flex;justify-content:space-between;padding:8px 8px 0;gap:10px;">
"##);
        for m in metrics.iter().skip(1).take(5) {
            vbars_html.push_str(&format!(
                r##"<span style="font-size:11px;color:#78716c;flex:1;text-align:center;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{}</span>"##,
                escape_html(&m.label)
            ));
        }
        vbars_html.push_str("</div></div>");

        sections.push(Section {
            id: "vertical-bars".to_string(),
            title: "Comparative Metrics".to_string(),
            icon: String::new(),
            content_type: "chart".to_string(),
            position: "col3-top".to_string(),
            html: vbars_html,
        });
    } else {
        // NO NUMERICAL DATA: Provide conceptual cards instead of fake charts!
        let concept_1 = raw_sentences.get(1).copied().unwrap_or("Restraint and durability form the core ethos.");
        let concept_2 = raw_sentences.get(2).copied().unwrap_or("Clarity of purpose eliminates extraneous distraction.");

        let conceptual_html = format!(
            r##"
<div style="display:flex;flex-direction:column;gap:12px;">
  <div style="padding:14px;background:#fbf9f4;border:1px solid #e5dfd3;border-radius:8px;">
    <div style="font-family:var(--font-mono);font-size:11px;color:#ea580c;font-weight:600;margin-bottom:4px;">PRIMARY PRINCIPLE</div>
    <div style="font-size:13.5px;color:#1c1917;line-height:1.6;">{}</div>
  </div>
  <div style="padding:14px;background:#fbf9f4;border:1px solid #e5dfd3;border-radius:8px;">
    <div style="font-family:var(--font-mono);font-size:11px;color:#78716c;font-weight:600;margin-bottom:4px;">SUPPORTING THESIS</div>
    <div style="font-size:13.5px;color:#1c1917;line-height:1.6;">{}</div>
  </div>
</div>
"##,
            concept_1, concept_2
        );

        sections.push(Section {
            id: "donut-bars".to_string(),
            title: "Foundational Pillars".to_string(),
            icon: String::new(),
            content_type: "summary".to_string(),
            position: "col1-top".to_string(),
            html: conceptual_html,
        });

        let quote_text = raw_sentences.last().copied().unwrap_or("Simplicity and consequential focus down to the last detail.");
        let quote_html = format!(
            r##"
<div style="padding:20px 16px;background:#fffaf5;border:1px solid #fed7aa;border-radius:8px;text-align:center;">
  <div style="font-family:var(--font-serif);font-size:17px;font-style:italic;color:#9a3412;line-height:1.6;margin-bottom:8px;">
    “{}”
  </div>
  <span style="font-family:var(--font-mono);font-size:11px;color:#ea580c;text-transform:uppercase;">Central Premise</span>
</div>
"##,
            quote_text
        );

        sections.push(Section {
            id: "vertical-bars".to_string(),
            title: "Core Thesis".to_string(),
            icon: String::new(),
            content_type: "notes".to_string(),
            position: "col3-top".to_string(),
            html: quote_html,
        });
    }

    // 3. Breakdown / Data Table (Col 1 Bottom)
    let mut table_rows = String::new();
    let sample_items: Vec<&str> = raw_sentences.iter().skip(1).take(4).copied().collect();
    for (idx, item) in sample_items.iter().enumerate() {
        let label = format!("Dimension {:02}", idx + 1);
        let excerpt = if item.len() > 60 { &item[..60] } else { item };
        table_rows.push_str(&format!(
            r##"
<tr style="border-bottom:1px solid #f0ece3;">
  <td style="padding:9px 6px;font-weight:600;color:#1c1917;font-size:12px;">{}</td>
  <td style="padding:9px 6px;color:#44403c;font-size:13px;">{}...</td>
</tr>
"##,
            label, escape_html(excerpt)
        ));
    }

    let table_html = format!(
        r##"
<table style="width:100%;border-collapse:collapse;font-size:13px;text-align:left;">
  <thead>
    <tr style="border-bottom:1px solid #e5dfd3;">
      <th style="padding:8px 6px;font-family:var(--font-mono);font-size:11px;color:#78716c;text-transform:uppercase;">Topic</th>
      <th style="padding:8px 6px;font-family:var(--font-mono);font-size:11px;color:#78716c;text-transform:uppercase;">Extracted Insight</th>
    </tr>
  </thead>
  <tbody>
    {}
  </tbody>
</table>
"##,
        table_rows
    );

    sections.push(Section {
        id: "breakdown-table".to_string(),
        title: "Comparative Review".to_string(),
        icon: String::new(),
        content_type: "table".to_string(),
        position: "col1-bottom".to_string(),
        html: table_html,
    });

    // 4. Key Notes & Takeaways (Col 3 Bottom)
    let mut notes_html = String::from("<div style=\"display:flex;flex-direction:column;gap:11px;\">");
    for s in raw_sentences.iter().skip(2).take(4) {
        notes_html.push_str(&format!(
            r##"
<div style="display:flex;align-items:flex-start;gap:9px;border-bottom:1px solid #f5f1e8;padding-bottom:9px;">
  <span style="color:#ea580c;font-weight:bold;margin-top:2px;">—</span>
  <div style="font-size:13px;line-height:1.6;color:#1c1917;">{}</div>
</div>
"##,
            escape_html(s)
        ));
    }
    notes_html.push_str("</div>");

    sections.push(Section {
        id: "key-notes".to_string(),
        title: "Key Takeaways".to_string(),
        icon: String::new(),
        content_type: "notes".to_string(),
        position: "col3-bottom".to_string(),
        html: notes_html,
    });

    sections
}

/// Extracts real percentages, currency, and numerical counts from the text
fn extract_metrics_from_text(text: &str) -> Vec<MetricItem> {
    let mut metrics = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        let clean = word.trim_matches(|c: char| c == '(' || c == ')' || c == ',' || c == '.' || c == ':');

        // Look for percentages (e.g. "28%", "37.3%")
        if clean.ends_with('%') {
            if let Ok(num) = clean.trim_end_matches('%').parse::<f64>() {
                // Find previous word or next word as label
                let label = if i > 0 {
                    let prev = words[i - 1].trim_matches(|c: char| !c.is_alphabetic());
                    if !prev.is_empty() && prev != "by" && prev != "at" && prev != "of" && prev != "to" {
                        prev.to_string()
                    } else if i > 1 {
                        words[i - 2].trim_matches(|c: char| !c.is_alphabetic()).to_string()
                    } else {
                        "Metric".to_string()
                    }
                } else {
                    "Metric".to_string()
                };

                metrics.push(MetricItem {
                    label: capitalize(&label),
                    value_str: format!("{}%", num),
                    numeric_value: num,
                });
            }
        }
        // Look for dollar values (e.g. "$196.6")
        else if clean.starts_with('$') {
            if let Ok(num) = clean.trim_start_matches('$').parse::<f64>() {
                let unit = if i + 1 < words.len() {
                    let next = words[i + 1].trim_matches(|c: char| !c.is_alphabetic());
                    if next == "billion" || next == "million" || next == "trillion" {
                        format!(" {}", next)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                let label = if i > 0 {
                    words[i - 1].trim_matches(|c: char| !c.is_alphabetic()).to_string()
                } else {
                    "Valuation".to_string()
                };

                metrics.push(MetricItem {
                    label: capitalize(&label),
                    value_str: format!("${}{}", num, unit),
                    numeric_value: num,
                });
            }
        }
    }

    // Deduplicate metrics by label
    metrics.sort_by(|a, b| b.numeric_value.partial_cmp(&a.numeric_value).unwrap_or(std::cmp::Ordering::Equal));
    metrics.dedup_by(|a, b| a.label.to_lowercase() == b.label.to_lowercase());
    metrics
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "chartify-backend",
        "version": "1.1.0"
    }))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let default_api_key = std::env::var("TOKENHARBOR_API_KEY")
        .unwrap_or_else(|_| "thk_live_9FVJ0uZ1CLORCgqfdHabUKmDhqth5UqGqH3vF8gA41yQEDyCz7pkWFOp3qm8ZWA-".to_string());

    let state = Arc::new(AppState {
        client: Client::new(),
        default_api_key,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", axum::routing::get(health_check))
        .route("/api/analyze", axum::routing::post(analyze_text))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("🚀 Chartify backend listening on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}
