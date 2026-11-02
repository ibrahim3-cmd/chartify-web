<script setup>
import { ref, onMounted, nextTick, watch } from 'vue'

const props = defineProps({
  section: {
    type: Object,
    required: true,
  },
  isTall: {
    type: Boolean,
    default: false,
  },
  index: {
    type: Number,
    default: 0,
  },
})

const emit = defineEmits(['expand'])

const contentRef = ref(null)
const isVisible = ref(false)

function renderHtml() {
  if (!contentRef.value || !props.section?.html) return
  contentRef.value.innerHTML = props.section.html

  // Run any scripts if included
  const scripts = contentRef.value.querySelectorAll('script')
  scripts.forEach((oldScript) => {
    const newScript = document.createElement('script')
    Array.from(oldScript.attributes).forEach((attr) =>
      newScript.setAttribute(attr.name, attr.value)
    )
    newScript.appendChild(document.createTextNode(oldScript.innerHTML))
    oldScript.parentNode.replaceChild(newScript, oldScript)
  })
}

onMounted(() => {
  setTimeout(() => {
    isVisible.value = true
  }, props.index * 50)

  nextTick(renderHtml)
})

watch(() => props.section?.html, () => {
  nextTick(renderHtml)
})
</script>

<template>
  <div
    :class="['studio-card', { 'is-tall': isTall, visible: isVisible }]"
  >
    <!-- Card Header -->
    <div class="card-top-bar">
      <div class="card-title-group">
        <h3 class="card-name">{{ section.title }}</h3>
      </div>

      <!-- Handcrafted zoom button with red/orange arrow hover from user's sketch -->
      <button
        class="card-zoom-btn"
        @click.stop="emit('expand')"
        :title="`Zoom into ${section.title}`"
        :aria-label="`Zoom into ${section.title}`"
      >
        <svg
          class="zoom-arrow-icon"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="15 3 21 3 21 9" />
          <polyline points="9 21 3 21 3 15" />
          <line x1="21" y1="3" x2="14" y2="10" />
          <line x1="3" y1="21" x2="10" y2="14" />
        </svg>
      </button>
    </div>

    <!-- Card Content -->
    <div class="card-content-area">
      <div ref="contentRef" class="paper-html-wrapper"></div>
    </div>
  </div>
</template>

<style scoped>
.studio-card {
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-card);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  opacity: 0;
  transform: translateY(8px);
  transition: opacity 0.3s ease, transform 0.3s ease, box-shadow 0.2s ease, border-color 0.2s ease;
}

.studio-card.visible {
  opacity: 1;
  transform: translateY(0);
}

.studio-card:hover {
  border-color: #d1c8b8;
  box-shadow: var(--shadow-card-hover);
}

.studio-card.is-tall {
  height: 100%;
  min-height: 480px;
}

/* Card Top Bar */
.card-top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px 12px;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-card-warm);
}

.card-title-group {
  display: flex;
  align-items: center;
}

.card-name {
  font-family: var(--font-serif);
  font-size: 15.5px;
  font-weight: 600;
  color: var(--ink-primary);
  letter-spacing: -0.2px;
}

/* Zoom button */
.card-zoom-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: var(--radius-xs);
  background: var(--bg-canvas-subtle);
  border: 1px solid var(--border-subtle);
  color: #dc2626; /* Handcrafted red accent like the sketch */
  cursor: pointer;
  transition: all 0.15s ease;
}

.zoom-arrow-icon {
  transition: transform 0.15s ease;
}

.card-zoom-btn:hover {
  background: #fef2f2;
  border-color: #fca5a5;
  color: #b91c1c;
  transform: scale(1.08);
}

.card-zoom-btn:hover .zoom-arrow-icon {
  transform: scale(1.15);
}

/* Card Content */
.card-content-area {
  padding: 18px 20px 22px;
  flex: 1;
  overflow-y: auto;
  max-height: 480px;
}

.paper-html-wrapper {
  font-family: var(--font-sans);
  font-size: 13.5px;
  line-height: 1.65;
  color: var(--ink-secondary);
}

/* Nested styling reset for cleanly rendered HTML */
:deep(table) {
  width: 100%;
  border-collapse: collapse;
}

:deep(th) {
  font-family: var(--font-mono);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--ink-muted);
}

:deep(td) {
  color: var(--ink-secondary);
}

:deep(p) {
  margin-bottom: 8px;
}

:deep(p:last-child) {
  margin-bottom: 0;
}
</style>
