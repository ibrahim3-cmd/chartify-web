# Chartify — Editorial Text Information Board

A handcrafted information synthesizer built with **Vue 3 + Vite** and a high-performance **Rust (Axum)** backend. Transforms text into a modular editorial board featuring authentic data charts, comparison tables, and concise takeaways.

## Aesthetic & Design Philosophy

- **Handcrafted Studio Feel**: Clean editorial layout inspired by architects' draft boards and Swiss graphic design.
- **Warm Parchment Canvas**: Yellowish-white paper background (`#fbf9f4`) with subtle organic texture.
- **Designer Orange Accents**: Warm terracotta & burnt orange buttons (`#ea580c`), tactile hover states, and warm borders (`#e5dfd3`).
- **Editorial Typography**: `Fraunces` serif display paired with `Plus Jakarta Sans` for body text and `JetBrains Mono` for metadata numbers.
- **No AI Slop**: Free of generic neon purple/cyan glowing blobs, robotic badges, and fabricated filler graphs.

## Dynamic & Authentic Data Visualizations

- **Strict Data Honesty**: Charts and tables are only rendered if the input text actually contains quantifiable metrics or structured comparisons.
- **Conditional Charts**: If percentages or numbers are present, generates proportional bars labeled directly from your text.
- **Narrative Fallback**: If the text is purely qualitative or philosophical, produces conceptual pillars, thesis quotes, and structured notes instead of hallucinated graphs.

## Quick Start

### 1. Run the Rust Backend

```bash
cd backend
cargo run
```

Backend runs on `http://localhost:3001`.

### 2. Run the Vue Frontend

```bash
cd frontend
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`.

### 3. Usage

1. Open `http://localhost:5173`.
2. Paste text or click a quick sample (e.g. *Market Metrics*, *Energy Transition*, or *Design Philosophy*).
3. Click the orange **Draft Information Board** button.
4. Click the **⤢** expand icon in the top-right corner of any card to zoom into a centered editorial sheet.
