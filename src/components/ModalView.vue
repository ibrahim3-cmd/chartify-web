<script setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue'

const props = defineProps({
  section: {
    type: Object,
    required: true,
  },
})

const emit = defineEmits(['close'])

const modalBodyRef = ref(null)
const copied = ref(false)

function renderContent() {
  if (!modalBodyRef.value || !props.section?.html) return
  modalBodyRef.value.innerHTML = props.section.html

  // Re-run scripts if any
  const scripts = modalBodyRef.value.querySelectorAll('script')
  scripts.forEach((oldScript) => {
    const newScript = document.createElement('script')
    Array.from(oldScript.attributes).forEach((attr) =>
      newScript.setAttribute(attr.name, attr.value)
    )
    newScript.appendChild(document.createTextNode(oldScript.innerHTML))
    oldScript.parentNode.replaceChild(newScript, oldScript)
  })
}

function onKeydown(e) {
  if (e.key === 'Escape') {
    emit('close')
  }
}

async function copyHtml() {
  if (!props.section?.html) return
  try {
    await navigator.clipboard.writeText(props.section.html)
    copied.value = true
    setTimeout(() => {
      copied.value = false
    }, 2000)
  } catch (err) {
    console.error('Failed to copy', err)
  }
}

onMounted(() => {
  nextTick(renderContent)
  window.addEventListener('keydown', onKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-sheet" role="dialog" aria-modal="true">
      <!-- Modal Top -->
      <div class="sheet-top-bar">
        <div class="sheet-title-col">
          <div class="sheet-meta">
            <span class="sheet-badge">{{ section.content_type || 'section' }}</span>
          </div>
          <h2 class="sheet-heading">{{ section.title }}</h2>
        </div>

        <div class="sheet-controls">
          <button
            class="copy-html-btn"
            @click="copyHtml"
            :title="copied ? 'Copied!' : 'Copy raw HTML'"
            type="button"
          >
            <span>{{ copied ? 'Copied ✓' : 'Copy HTML' }}</span>
          </button>

          <button
            class="close-sheet-btn"
            @click="emit('close')"
            aria-label="Close modal"
            title="Press Esc to exit"
            type="button"
          >
            ✕
          </button>
        </div>
      </div>

      <!-- Modal Body -->
      <div class="sheet-scroll-body">
        <div ref="modalBodyRef" class="sheet-rendered-html"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: var(--bg-modal-overlay);
  backdrop-filter: blur(6px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 24px;
}

.modal-sheet {
  width: 100%;
  max-width: 900px;
  max-height: 88vh;
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-modal);
  animation: sheet-pop 0.22s ease-out;
}

@keyframes sheet-pop {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.sheet-top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 22px 28px;
  border-bottom: 1px solid var(--border-card);
  background: var(--bg-card-warm);
  flex-shrink: 0;
}

.sheet-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.sheet-badge {
  font-family: var(--font-mono);
  font-size: 11px;
  text-transform: uppercase;
  color: var(--ink-muted);
  letter-spacing: 0.6px;
  background: var(--bg-canvas-subtle);
  padding: 2px 8px;
  border-radius: 4px;
  border: 1px solid var(--border-subtle);
}

.sheet-heading {
  font-family: var(--font-serif);
  font-size: 24px;
  font-weight: 600;
  color: var(--ink-primary);
  line-height: 1.2;
}

.sheet-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.copy-html-btn {
  padding: 7px 14px;
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-xs);
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--ink-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.copy-html-btn:hover {
  background: var(--bg-canvas-subtle);
  color: var(--ink-primary);
  border-color: var(--ink-muted);
}

.close-sheet-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-canvas-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xs);
  font-size: 14px;
  color: var(--ink-muted);
  cursor: pointer;
  transition: all 0.15s ease;
}

.close-sheet-btn:hover {
  background: #fee2e2;
  color: #dc2626;
  border-color: #fca5a5;
}

.sheet-scroll-body {
  padding: 32px 36px;
  overflow-y: auto;
  flex: 1;
}

.sheet-rendered-html {
  font-family: var(--font-sans);
  font-size: 15px;
  line-height: 1.7;
  color: var(--ink-secondary);
}

@media (max-width: 640px) {
  .modal-backdrop {
    padding: 12px;
  }
  .sheet-top-bar {
    padding: 16px 20px;
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  .sheet-controls {
    align-self: flex-end;
  }
  .sheet-scroll-body {
    padding: 20px;
  }
}
</style>
