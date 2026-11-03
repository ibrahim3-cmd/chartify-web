<script setup>
import { ref } from 'vue'
import TextInput from './components/TextInput.vue'
import Dashboard from './components/Dashboard.vue'

const sections = ref([])
const apiNotice = ref('')
const isLoading = ref(false)
const errorMessage = ref('')
const showDashboard = ref(false)

async function handleAnalyze({ text, model, apiKey }) {
  isLoading.value = true
  errorMessage.value = ''
  sections.value = []
  apiNotice.value = ''
  showDashboard.value = false

  try {
    const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || (import.meta.env.DEV ? 'http://localhost:3001' : '')

    if (!apiBaseUrl) {
      throw new Error('The backend URL is not configured. Set VITE_API_BASE_URL before building.')
    }

    const res = await fetch(`${apiBaseUrl.replace(/\/+$/, '')}/api/analyze`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        text,
        model,
        api_key: apiKey || undefined,
      }),
    })

    const data = await res.json()

    if (!res.ok) {
      throw new Error(data.error || `Error (${res.status})`)
    }

    if (data.sections && data.sections.length > 0) {
      sections.value = data.sections
      apiNotice.value = data.api_notice || ''
      showDashboard.value = true
    } else {
      throw new Error('No sections generated for this text.')
    }
  } catch (err) {
    errorMessage.value = err.message || 'Unable to connect to the backend server.'
  } finally {
    isLoading.value = false
  }
}

function handleBack() {
  showDashboard.value = false
  sections.value = []
  apiNotice.value = ''
  errorMessage.value = ''
}
</script>

<template>
  <div class="app-layout">
    <!-- Studio Header -->
    <header class="app-header">
      <div class="header-inner">
        <div class="brand-mark" @click="handleBack" style="cursor: pointer;">
          <div class="brand-square">
            <span class="square-glyph">✦</span>
          </div>
          <div>
            <h1 class="brand-title">Chartify</h1>
            <p class="brand-caption">Editorial Text Information Board</p>
          </div>
        </div>

        <div v-if="showDashboard" class="header-actions">
          <button class="nav-back-button" @click="handleBack">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
              <path d="M19 12H5" />
              <path d="M12 19l-7-7 7-7" />
            </svg>
            
          </button>
        </div>
      </div>
    </header>

    <!-- Main Workspace -->
    <main class="app-main">
      <Transition name="fade" mode="out-in">
        <TextInput
          v-if="!showDashboard"
          :is-loading="isLoading"
          :error-message="errorMessage"
          @analyze="handleAnalyze"
        />
        <Dashboard
          v-else
          :sections="sections"
          :api-notice="apiNotice"
          @back="handleBack"
        />
      </Transition>
    </main>

    <!-- Minimal Studio Footer -->
    <footer class="app-footer">
      <div class="footer-inner">
      </div>
    </footer>
  </div>
</template>

<style scoped>
.app-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-canvas);
}

.app-header {
  border-bottom: 1px solid var(--border-card);
  background: rgba(251, 249, 244, 0.94);
  backdrop-filter: blur(8px);
  position: sticky;
  top: 0;
  z-index: 50;
  padding: 16px 0;
}

.header-inner {
  max-width: 1420px;
  margin: 0 auto;
  padding: 0 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.brand-mark {
  display: flex;
  align-items: center;
  gap: 12px;
}

.brand-square {
  width: 34px;
  height: 34px;
  background: var(--orange-primary);
  color: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  font-size: 16px;
  box-shadow: 0 2px 8px var(--orange-shadow);
}

.square-glyph {
  transform: translateY(-0.5px);
}

.brand-title {
  font-family: var(--font-serif);
  font-size: 22px;
  font-weight: 600;
  color: var(--ink-primary);
  line-height: 1.1;
  letter-spacing: -0.3px;
}

.brand-caption {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--ink-muted);
  text-transform: uppercase;
  letter-spacing: 0.8px;
  margin-top: 1px;
}

.nav-back-button {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-sm);
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 600;
  color: var(--ink-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.nav-back-button:hover {
  background: var(--bg-card-hover);
  border-color: var(--ink-primary);
  color: var(--ink-primary);
}

.app-main {
  flex: 1;
}

.app-footer {
  border-top: 1px solid var(--border-subtle);
  padding: 20px 0;
  background: transparent;
}

.footer-inner {
  max-width: 1420px;
  margin: 0 auto;
  padding: 0 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-family: var(--font-mono);
  font-size: 11.5px;
  color: var(--ink-muted);
}

@media (max-width: 640px) {
  .header-inner,
  .footer-inner {
    padding: 0 18px;
  }
}
</style>
