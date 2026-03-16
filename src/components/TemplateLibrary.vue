<template>
  <section class="template-library">
    <div class="header-row">
      <h3>{{ t("templates.title") }}</h3>
      <div class="header-actions">
        <span class="count">{{ visibleTemplates.length }}</span>
        <button type="button" @click="$emit('create-new-template')">{{ t("templates.new") }}</button>
        <button type="button" @click="$emit('import-template-bundle')">{{ t("templates.importBundle") }}</button>
        <button type="button" @click="$emit('export-template-bundle')">{{ t("templates.exportBundle") }}</button>
      </div>
    </div>

    <div class="filters">
      <label>
        {{ t("templates.platform") }}
        <select :value="platformFilter" @change="emitPlatformFilter">
          <option value="all">{{ t("templates.allPlatforms") }}</option>
          <option value="linux-shell">Linux shell</option>
          <option value="wsl">WSL</option>
          <option value="windows-powershell">PowerShell</option>
        </select>
      </label>
      <label>
        {{ t("templates.category") }}
        <select :value="categoryFilter" @change="emitCategoryFilter">
          <option value="all">{{ t("templates.allCategories") }}</option>
          <option v-for="category in categories" :key="category" :value="category">{{ category }}</option>
        </select>
      </label>
    </div>

    <p v-if="visibleTemplates.length === 0" class="empty">{{ t("templates.empty") }}</p>

    <div v-else class="template-list">
      <article
        v-for="template in visibleTemplates"
        :key="template.id"
        class="template-card"
        :class="{ active: template.id === selectedTemplateId }"
      >
        <div class="card-top">
          <div>
            <h4>{{ template.name }}</h4>
            <p>{{ template.description }}</p>
          </div>
          <span class="pill">{{ template.platformKind }}</span>
        </div>
        <div class="meta-row">
          <span>{{ template.category ?? t("templates.uncategorized") }}</span>
          <span v-if="template.builtIn">{{ t("templates.builtin") }}</span>
        </div>
        <code>{{ template.commandPattern }}</code>
        <ul v-if="template.params.length > 0" class="param-hints">
          <li v-for="param in template.params" :key="param.id">
            <strong>{{ param.label || param.paramKey }}</strong>
            <span>{{ param.paramKey }}</span>
            <span>{{ param.type }}</span>
          </li>
        </ul>
        <div class="card-actions">
          <button type="button" @click="$emit('select-template', template)">{{ t("templates.manage") }}</button>
          <button type="button" :disabled="!selectedNodeId" @click="$emit('apply-template', template)">
            {{ selectedNodeId ? t("templates.apply") : t("templates.selectNodeFirst") }}
          </button>
        </div>
      </article>
    </div>
  </section>
</template>

<script setup lang="ts">
import type { PlatformKind, TemplateDefinition } from "../domain/models";
import { useI18n } from "../i18n";

const { t } = useI18n();

defineProps<{
  visibleTemplates: TemplateDefinition[];
  categories: string[];
  platformFilter: PlatformKind | "all";
  categoryFilter: string;
  selectedNodeId: string | null;
  selectedTemplateId: string | null;
}>();

const emit = defineEmits<{
  (event: "update-platform-filter", value: PlatformKind | "all"): void;
  (event: "update-category-filter", value: string): void;
  (event: "select-template", template: TemplateDefinition): void;
  (event: "apply-template", template: TemplateDefinition): void;
  (event: "create-new-template"): void;
  (event: "import-template-bundle"): void;
  (event: "export-template-bundle"): void;
}>();

const emitPlatformFilter = (event: Event): void => {
  emit("update-platform-filter", (event.target as HTMLSelectElement).value as PlatformKind | "all");
};

const emitCategoryFilter = (event: Event): void => {
  emit("update-category-filter", (event.target as HTMLSelectElement).value);
};
</script>

<style scoped>
.template-library {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.header-row,
.card-top,
.meta-row {
  display: flex;
  justify-content: space-between;
  gap: 8px;
}

.header-row {
  align-items: center;
}

.header-row h3,
.card-top h4 {
  margin: 0;
}

.count,
.pill {
  border: 1px solid #3f5b80;
  border-radius: 999px;
  padding: 2px 8px;
  font-size: 11px;
  color: #a5bed8;
}

.header-actions,
.card-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.filters {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 4px;
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

.empty {
  margin: 0;
  color: #a5bed8;
}

.template-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.template-card {
  border: 1px solid #2e435f;
  border-radius: 12px;
  background: rgba(20, 38, 60, 0.45);
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.template-card.active {
  border-color: #90caf9;
}

.card-top p,
.meta-row {
  margin: 0;
  color: #a5bed8;
  font-size: 12px;
}

code {
  white-space: pre-wrap;
  word-break: break-word;
  background: #0f1620;
  border-radius: 8px;
  padding: 8px;
  color: #d3e8ff;
}

.param-hints {
  margin: 0;
  padding-left: 18px;
  color: #a5bed8;
  font-size: 12px;
}
</style>