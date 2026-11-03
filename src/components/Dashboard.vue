<script setup>
import { ref, computed } from 'vue'
import BoardSection from './BoardSection.vue'
import ModalView from './ModalView.vue'

const props = defineProps({
  sections: {
    type: Array,
    default: () => [],
  },
  apiNotice: {
    type: String,
    default: '',
  },
})

const emit = defineEmits(['back'])

const activeModalSection = ref(null)

function openModal(sec) {
  activeModalSection.value = sec
  document.body.style.overflow = 'hidden'
}

function closeModal() {
  activeModalSection.value = null
  document.body.style.overflow = ''
}

// Map sections into the 3 columns matching the user's drawing:
// If sections have position specified, use them; otherwise distribute evenly.
const col1Top = computed(() => {
  return (
    props.sections.find((s) => s.position === 'col1-top' || s.id === 'donut-bars') ||
    props.sections[0] ||
    null
  )
})

const col1Bottom = computed(() => {
  return (
    props.sections.find((s) => s.position === 'col1-bottom' || s.id === 'breakdown-table') ||
    (props.sections.length > 3 ? props.sections[3] : null)
  )
})

const col2Center = computed(() => {
  return (
    props.sections.find((s) => s.position === 'col2-center' || s.id === 'center-stack') ||
    props.sections[1] ||
    null
  )
})

const col3Top = computed(() => {
  return (
    props.sections.find((s) => s.position === 'col3-top' || s.id === 'vertical-bars') ||
    props.sections[2] ||
    null
  )
})

const col3Bottom = computed(() => {
  return (
    props.sections.find((s) => s.position === 'col3-bottom' || s.id === 'key-notes') ||
    (props.sections.length > 4 ? props.sections[4] : null)
  )
})

// Any extra sections beyond the mapped 5
const extraSections = computed(() => {
  const mappedIds = new Set([
    col1Top.value?.id,
    col1Bottom.value?.id,
    col2Center.value?.id,
    col3Top.value?.id,
    col3Bottom.value?.id,
  ].filter(Boolean))

  return props.sections.filter((s) => !mappedIds.has(s.id))
})
</script>

<template>
  <div class="dashboard-board-canvas">
    <!-- Top Studio Controls -->
    <div class="board-action-bar">
      <div class="bar-left">
        <button class="back-link-btn" @click="emit('back')">
          <span class="back-arrow">←</span>
          <span>Paste New Text</span>
        </button>
        <span class="card-count-indicator">
          {{ sections.length }} cards drafted
        </span>
      </div>

      <div class="bar-right">
        <span class="zoom-guide-hint">
          Click <strong style="color: #dc2626;">⤢</strong> in the top corner of any card to zoom
        </span>
      </div>
    </div>

    <!-- Quiet notice banner if API quota info -->
    <div v-if="apiNotice" class="studio-notice-bar">
      <span class="notice-symbol">ℹ</span>
      <span>{{ apiNotice }}</span>
    </div>

    <!-- 3-Column Board Canvas (Directly matching the sketch) -->
    <div class="three-col-board">
      <!-- Column 1 (Left) -->
      <div class="board-col col-left">
        <BoardSection
          v-if="col1Top"
          :section="col1Top"
          :index="0"
          @expand="openModal(col1Top)"
        />
        <BoardSection
          v-if="col1Bottom"
          :section="col1Bottom"
          :index="1"
          @expand="openModal(col1Bottom)"
        />
      </div>

      <!-- Column 2 (Center / Tall column) -->
      <div class="board-col col-center">
        <BoardSection
          v-if="col2Center"
          :section="col2Center"
          :is-tall="true"
          :index="2"
          @expand="openModal(col2Center)"
        />
      </div>

      <!-- Column 3 (Right) -->
      <div class="board-col col-right">
        <BoardSection
          v-if="col3Top"
          :section="col3Top"
          :index="3"
          @expand="openModal(col3Top)"
        />
        <BoardSection
          v-if="col3Bottom"
          :section="col3Bottom"
          :index="4"
          @expand="openModal(col3Bottom)"
        />
      </div>
    </div>

    <!-- Any extra cards generated -->
    <div v-if="extraSections.length > 0" class="extra-cards-row">
      <BoardSection
        v-for="(extra, idx) in extraSections"
        :key="extra.id"
        :section="extra"
        :index="5 + idx"
        @expand="openModal(extra)"
      />
    </div>

    <!-- Centered Modal View -->
    <Transition name="fade">
      <ModalView
        v-if="activeModalSection"
        :section="activeModalSection"
        @close="closeModal"
      />
    </Transition>
  </div>
</template>

<style scoped>
.dashboard-board-canvas {
  max-width: 1460px;
  margin: 0 auto;
  padding: 28px 32px 80px;
}

/* Action Bar */
.board-action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 22px;
  flex-wrap: wrap;
  gap: 14px;
}

.bar-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.back-link-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
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
  box-shadow: var(--shadow-card);
}

.back-link-btn:hover {
  background: var(--bg-card-hover);
  color: var(--ink-primary);
  border-color: var(--ink-muted);
}

.back-arrow {
  font-size: 15px;
  transition: transform 0.15s ease;
}

.back-link-btn:hover .back-arrow {
  transform: translateX(-2px);
}

.card-count-indicator {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--ink-muted);
}

.zoom-guide-hint {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--ink-muted);
}

/* Notice Bar */
.studio-notice-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  background: var(--orange-soft);
  border: 1px solid var(--orange-border);
  border-radius: var(--radius-sm);
  font-size: 13px;
  color: #9a3412;
  margin-bottom: 22px;
}

.notice-symbol {
  font-weight: 700;
  font-size: 14px;
}

/* 3-Column Board Canvas */
.three-col-board {
  display: grid;
  grid-template-columns: 1fr 1.05fr 1fr;
  gap: 22px;
  align-items: stretch;
}

.board-col {
  display: flex;
  flex-direction: column;
  gap: 22px;
}

.col-center {
  height: 100%;
}

.extra-cards-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 22px;
  margin-top: 22px;
}

@media (max-width: 1100px) {
  .three-col-board {
    grid-template-columns: 1fr 1fr;
  }
  .col-center {
    grid-column: span 2;
  }
}

@media (max-width: 768px) {
  .dashboard-board-canvas {
    padding: 16px;
  }
  .three-col-board {
    grid-template-columns: 1fr;
  }
  .col-center {
    grid-column: span 1;
  }
  .board-action-bar {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
