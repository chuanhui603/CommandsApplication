<template>
  <section class="output-preview">
    <div class="header-row">
      <h3>{{ t("preview.title") }}</h3>
      <div class="controls">
        <select :value="target" @change="emitTarget">
          <option value="linux-shell">Linux shell</option>
          <option value="wsl">WSL</option>
          <option value="windows-powershell">PowerShell</option>
        </select>
        <select :value="outputMode" @change="emitOutputMode">
          <option value="command">{{ t("preview.commandMode") }}</option>
          <option value="script">{{ t("preview.scriptMode") }}</option>
        </select>
        <button type="button" @click="$emit('generate')">{{ t("preview.generate") }}</button>
      </div>
    </div>

    <p class="hint">{{ t("preview.hint") }} {{ targetLabel }}</p>
    <ul v-if="diagnostics.length > 0" class="warnings">
      <li v-for="diagnostic in diagnostics" :key="`${diagnostic.code}-${diagnostic.message}`">{{ diagnostic.message }}</li>
    </ul>

    <pre>{{ content || t("preview.empty") }}</pre>

    <div v-if="recentResults.length > 0" class="history">
      <h4>{{ t("preview.history") }}</h4>
      <ul>
        <li v-for="result in recentResults" :key="result.id">
          <strong>{{ result.target }}</strong>
          <span>v{{ result.mindmapVersion }}</span>
          <span>{{ result.outputMode }}</span>
          <span>{{ result.createdAt }}</span>
        </li>
      </ul>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "../i18n";
import type { BuildResult, OutputMode, PlatformKind, ValidatorDiagnostic } from "../domain/models";

const { t } = useI18n();

const props = defineProps<{
  content: string;
  diagnostics: ValidatorDiagnostic[];
  target: PlatformKind;
  outputMode: OutputMode;
  recentResults: BuildResult[];
}>();

const emit = defineEmits<{
  (event: "update-target", target: PlatformKind): void;
  (event: "update-output-mode", outputMode: OutputMode): void;
  (event: "generate"): void;
}>();

const targetLabel = computed(() => props.target ?? t("preview.noTarget"));

const emitTarget = (event: Event): void => {
  const target = (event.target as HTMLSelectElement).value as PlatformKind;
  emit("update-target", target);
};

const emitOutputMode = (event: Event): void => {
  const outputMode = (event.target as HTMLSelectElement).value as OutputMode;
  emit("update-output-mode", outputMode);
};
</script>

<style scoped>
.output-preview {
  border: 1px solid #2e435f;
  border-radius: 12px;
  background: rgba(20, 38, 60, 0.55);
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.header-row {
  display: flex;
  justify-content: space-between;
  gap: 8px;
}

.controls {
  display: flex;
  gap: 8px;
}

.header-row h3,
.warnings {
  margin: 0;
}

.hint,
.warnings {
  color: #a5bed8;
  font-size: 12px;
}

select,
button {
  border: 1px solid #2f3f54;
  background: #101722;
  color: #d3e8ff;
  border-radius: 6px;
  padding: 8px;
}

pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  background: #0f1620;
  border-radius: 8px;
  padding: 10px;
  color: #d3e8ff;
  min-height: 88px;
}

.history h4,
.history ul {
  margin: 0;
}

.history ul {
  padding-left: 18px;
  color: #a5bed8;
  font-size: 12px;
}
</style>