<script setup>
import { ref } from 'vue'

const props = defineProps({
  isLoading: Boolean,
  errorMessage: String,
})

const emit = defineEmits(['analyze'])

const text = ref('')
const selectedModel = ref('deepseek-v4.1-flash:free')
const showApiKeyInput = ref(false)
const customApiKey = ref('')

const models = [
  { id: 'deepseek-v4.1-flash:free', label: 'DeepSeek Flash' },
  { id: 'mimo-v2.6-flash:free', label: 'MiMo Flash' },
]

const samples = [
  {
    name: 'Numerical: Market & Metrics',
    snippet: `The global artificial intelligence market was valued at $196.6 billion in 2023 and is projected to reach $1,811.8 billion by 2030, growing at a CAGR of 37.3%. Key sectors driving adoption include healthcare (28%), finance (22%), retail (15%), manufacturing (14%), and transportation (12%). The United States leads with 38% market share, followed by China at 24%, the UK at 8%, and Germany at 6%. Notable trends include generative AI adoption increasing from 5% to 67% of enterprises between 2022 and 2024, with average investment per company rising from $250K to $2.1M. Cloud-based AI solutions account for 62% of deployments, while edge AI is growing at 45% annually. Key challenges remain: data privacy concerns (cited by 71% of organizations), talent shortage (64%), and integration complexity (58%).`,
  },
  {
    name: 'Numerical: Energy Transition',
    snippet: `Global renewable energy installations expanded by 50% in 2023, adding 510 GW of capacity worldwide. Solar photovoltaic accounted for 75% of new additions, wind power provided 21%, and hydro contributed 4%. Investment reached $1.77 trillion across clean technologies, representing an 18% annual expansion. 14 million new electric vehicles were sold globally, capturing 18% of total vehicle sales. Grid modernization attracted $330 billion in infrastructure capital. Battery storage deployments tripled to 44 GW. Sub-Saharan Africa saw 41% microgrid growth, while Southeast Asia saw 34%.`,
  },
  {
    name: 'Qualitative: Architectural Philosophy',
    snippet: `Dieter Rams formulated ten principles for good design centered on restraint, clarity, and durability. Good design is innovative; technological development offers new opportunities for original design. It makes a product useful, emphasizing utility while disregarding anything that detracts from it. Good design is aesthetic; the aesthetic quality of an object is integral to its usefulness because objects used daily influence our well-being. Good design makes a product understandable, clarifying its structure and enabling intuition without instructions. Above all, good design is unobtrusive, honest, durable, and consequential down to the last detail. It is as little design as possible, returning to purity and simplicity.`,
  },
]

function handleSubmit() {
  if (!text.value.trim() || props.isLoading) return
  emit('analyze', {
    text: text.value.trim(),
    model: selectedModel.value,
    apiKey: customApiKey.value.trim() || null,
  })
}

function loadSample(sample) {
  text.value = sample.snippet
}
</script>

<template>
  <div class="workspace-wrapper">
    <div class="workspace-card">
      <!-- Editorial Title -->
      <div class="intro-block">
        <h2 class="intro-heading">Paste text. Build an information board.</h2>
        <p class="intro-description">
          Synthesizes text into concise notes, structured data tables, and relatable charts
          strictly when numbers exist — avoiding fabricated or irrelevant graphs.
        </p>
      </div>

      <!-- Controls Row: Model Picker & Samples -->
      <div class="controls-bar">
        <div class="model-picker-group">
          <span class="control-label">Model</span>
          <div class="segmented-picker">
            <button
              v-for="m in models"
              :key="m.id"
              :class="['segment-btn', { active: selectedModel === m.id }]"
              @click="selectedModel = m.id"
              type="button"
            >
              {{ m.label }}
            </button>
          </div>
        </div>

        <div class="sample-picker-group">
          <span class="control-label">Quick Sample</span>
          <div class="sample-chips">
            <button
              v-for="s in samples"
              :key="s.name"
              class="sample-chip-btn"
              @click="loadSample(s)"
              type="button"
            >
              {{ s.name }}
            </button>
          </div>
        </div>
      </div>

      <!-- Textarea Card -->
      <div class="text-editor-box">
        <textarea
          id="raw-text-input"
          v-model="text"
          rows="11"
          placeholder="Paste articles, meeting notes, financial reports, research documents, or qualitative essays..."
          :disabled="isLoading"
        ></textarea>

        <div class="editor-status-bar">
          <span class="char-tally">{{ text.length.toLocaleString() }} characters</span>
          <button
            v-if="text.length > 0"
            class="text-clear-action"
            @click="text = ''"
            type="button"
          >
            Clear
          </button>
        </div>
      </div>

      <!-- Optional API Key -->
      <div class="optional-config">
        <button
          class="config-toggle-btn"
          @click="showApiKeyInput = !showApiKeyInput"
          type="button"
        >
          <span class="toggle-icon">{{ showApiKeyInput ? '−' : '+' }}</span>
          <span>Custom Token Harbor API Key (Optional)</span>
        </button>

        <Transition name="fade">
          <div v-if="showApiKeyInput" class="config-drawer">
            <input
              v-model="customApiKey"
              type="password"
              placeholder="thk_live_..."
              class="key-text-input"
            />
            <span class="key-hint">If left empty, your configured workspace key is used.</span>
          </div>
        </Transition>
      </div>

      <!-- Error notice -->
      <Transition name="fade">
        <div v-if="errorMessage" class="error-strip">
          <span class="error-dot">●</span>
          <span>{{ errorMessage }}</span>
        </div>
      </Transition>

      <!-- Primary Orange Action Button -->
      <button
        id="generate-board-btn"
        class="primary-orange-btn"
        @click="handleSubmit"
        :disabled="!text.trim() || isLoading"
        type="button"
      >
        <template v-if="isLoading">
          <span class="btn-spinner"></span>
          <span>Synthesizing text & drafting cards...</span>
        </template>
        <template v-else>
          <span>Draft Information Board</span>
          <span class="btn-arrow">→</span>
        </template>
      </button>
    </div>
  </div>
</template>

<style scoped>
.workspace-wrapper {
  max-width: 900px;
  margin: 0 auto;
  padding: 44px 24px 80px;
}

.workspace-card {
  display: flex;
  flex-direction: column;
  gap: 26px;
}

/* Editorial Title */
.intro-block {
  text-align: left;
  border-bottom: 1px solid var(--border-card);
  padding-bottom: 22px;
}

.intro-heading {
  font-family: var(--font-serif);
  font-size: 38px;
  font-weight: 600;
  color: var(--ink-primary);
  line-height: 1.18;
  letter-spacing: -0.8px;
  margin-bottom: 10px;
}

.intro-description {
  font-size: 15.5px;
  color: var(--ink-secondary);
  line-height: 1.6;
  max-width: 680px;
}

/* Controls Bar */
.controls-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  flex-wrap: wrap;
}

.control-label {
  font-family: var(--font-mono);
  font-size: 11px;
  text-transform: uppercase;
  color: var(--ink-muted);
  letter-spacing: 0.6px;
  margin-bottom: 6px;
  display: block;
}

.model-picker-group {
  display: flex;
  flex-direction: column;
}

.segmented-picker {
  display: inline-flex;
  background: var(--bg-canvas-subtle);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-sm);
  padding: 3px;
  gap: 3px;
}

.segment-btn {
  padding: 6px 14px;
  background: transparent;
  border: none;
  border-radius: var(--radius-xs);
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 500;
  color: var(--ink-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.segment-btn:hover {
  color: var(--ink-primary);
}

.segment-btn.active {
  background: var(--bg-card);
  color: var(--ink-primary);
  font-weight: 600;
  box-shadow: 0 1px 3px rgba(41, 37, 36, 0.08);
}

.sample-picker-group {
  display: flex;
  flex-direction: column;
}

.sample-chips {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.sample-chip-btn {
  padding: 6px 12px;
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-xs);
  font-family: var(--font-sans);
  font-size: 12px;
  font-weight: 500;
  color: var(--ink-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.sample-chip-btn:hover {
  background: var(--orange-soft);
  border-color: var(--orange-border);
  color: var(--orange-primary);
}

/* Text Editor */
.text-editor-box {
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-card);
  overflow: hidden;
  transition: border-color 0.15s ease, box-shadow 0.15s ease;
}

.text-editor-box:focus-within {
  border-color: var(--orange-primary);
  box-shadow: 0 0 0 3px var(--orange-shadow), var(--shadow-card);
}

textarea {
  width: 100%;
  padding: 20px 22px;
  background: transparent;
  border: none;
  font-family: var(--font-sans);
  font-size: 14.5px;
  line-height: 1.7;
  color: var(--ink-primary);
  resize: vertical;
  min-height: 230px;
  outline: none;
}

textarea::placeholder {
  color: var(--ink-faint);
}

.editor-status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 20px;
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-card-warm);
}

.char-tally {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--ink-muted);
}

.text-clear-action {
  background: transparent;
  border: none;
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--ink-muted);
  cursor: pointer;
}

.text-clear-action:hover {
  color: #dc2626;
}

/* Optional API Config */
.optional-config {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.config-toggle-btn {
  background: transparent;
  border: none;
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--ink-muted);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  align-self: flex-start;
  padding: 4px 0;
}

.config-toggle-btn:hover {
  color: var(--ink-primary);
}

.toggle-icon {
  font-weight: 700;
  font-size: 14px;
}

.config-drawer {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 14px;
  background: var(--bg-canvas-subtle);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-sm);
}

.key-text-input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-xs);
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--ink-primary);
  outline: none;
}

.key-text-input:focus {
  border-color: var(--orange-primary);
}

.key-hint {
  font-size: 11px;
  color: var(--ink-muted);
}

/* Error Strip */
.error-strip {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: var(--radius-sm);
  color: #b91c1c;
  font-size: 13px;
}

.error-dot {
  font-size: 10px;
}

/* Handcrafted Designer Orange Button */
.primary-orange-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 16px 28px;
  background: var(--orange-primary);
  border: 1px solid var(--orange-hover);
  border-radius: var(--radius-sm);
  color: #ffffff;
  font-family: var(--font-sans);
  font-size: 15px;
  font-weight: 600;
  letter-spacing: -0.2px;
  cursor: pointer;
  transition: all 0.15s ease;
  box-shadow: 0 2px 6px var(--orange-shadow), 0 1px 2px rgba(0, 0, 0, 0.05);
}

.primary-orange-btn:hover:not(:disabled) {
  background: var(--orange-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px var(--orange-shadow);
}

.primary-orange-btn:active:not(:disabled) {
  transform: translateY(0);
  background: var(--orange-active);
}

.primary-orange-btn:disabled {
  opacity: 0.55;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.btn-arrow {
  font-size: 17px;
  transition: transform 0.15s ease;
}

.primary-orange-btn:hover:not(:disabled) .btn-arrow {
  transform: translateX(3px);
}

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.35);
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@media (max-width: 640px) {
  .intro-heading {
    font-size: 28px;
  }
  .controls-bar {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
