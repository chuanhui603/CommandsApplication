<template>
  <section class="template-editor">
    <div class="header-row">
      <h3>{{ headerLabel }}</h3>
      <div class="header-actions">
        <button type="button" @click="$emit('create-new')">{{ t("templates.new") }}</button>
        <button v-if="template" type="button" @click="$emit('cancel-edit')">{{ t("templates.cancel") }}</button>
      </div>
    </div>

    <template v-if="template?.builtIn">
      <p class="readonly-note">{{ t("templates.readonly") }}</p>
      <label>
        {{ t("templates.cloneName") }}
        <input v-model="cloneName" />
      </label>
      <button type="button" @click="emitClone" :disabled="!cloneName.trim()">{{ t("templates.clone") }}</button>
    </template>

    <template v-else>
      <label>
        {{ t("templates.name") }}
        <input v-model="draft.name" />
      </label>
      <label>
        {{ t("templates.description") }}
        <textarea v-model="draft.description" />
      </label>
      <div class="grid-two">
        <label>
          {{ t("templates.platform") }}
          <select v-model="draft.platformKind">
            <option value="linux-shell">Linux shell</option>
            <option value="wsl">WSL</option>
            <option value="windows-powershell">PowerShell</option>
          </select>
        </label>
        <label>
          {{ t("templates.category") }}
          <input v-model="draft.category" />
        </label>
      </div>
      <label>
        {{ t("templates.commandPattern") }}
        <textarea v-model="draft.commandPattern" />
      </label>

      <div class="params-block">
        <div class="header-row">
          <h4>{{ t("templates.paramDefinitions") }}</h4>
          <button type="button" @click="addParam">{{ t("templates.addParam") }}</button>
        </div>
        <div v-for="(param, index) in draft.params" :key="param.id" class="param-card">
          <div class="grid-two">
            <label>
              {{ t("templates.paramLabel") }}
              <input v-model="param.label" />
            </label>
            <label>
              {{ t("templates.paramKey") }}
              <input v-model="param.paramKey" />
            </label>
          </div>
          <div class="grid-three">
            <label>
              {{ t("templates.paramType") }}
              <select v-model="param.type">
                <option value="text">text</option>
                <option value="boolean">boolean</option>
                <option value="single-select">single-select</option>
                <option value="path">path</option>
              </select>
            </label>
            <label>
              {{ t("templates.defaultValue") }}
              <input v-model="param.defaultValue" />
            </label>
            <label class="checkbox">
              <input v-model="param.required" type="checkbox" />
              {{ t("templates.required") }}
            </label>
          </div>
          <label v-if="param.type === 'single-select'">
            {{ t("templates.paramOptions") }}
            <input v-model="param.optionsInput" :placeholder="String(t('templates.paramOptionsHint'))" />
          </label>
          <button type="button" class="danger" @click="removeParam(index)">{{ t("templates.removeParam") }}</button>
        </div>
      </div>

      <div class="footer-actions">
        <button type="button" @click="emitSave">{{ saveLabel }}</button>
        <button v-if="template?.id" type="button" class="danger" @click="$emit('delete-template', template.id)">
          {{ t("templates.delete") }}
        </button>
      </div>
    </template>
  </section>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { TemplateDefinition, TemplateParamDefinition } from "../domain/models";
import { useI18n } from "../i18n";

interface TemplateDraftParam extends TemplateParamDefinition {
  optionsInput: string;
}

interface TemplateDraft {
  templateId?: string;
  name: string;
  description: string;
  platformKind: TemplateDefinition["platformKind"];
  category: string;
  commandPattern: string;
  params: TemplateDraftParam[];
}

const props = defineProps<{
  template: TemplateDefinition | null;
}>();

const emit = defineEmits<{
  (event: "save-template", payload: {
    templateId?: string;
    name: string;
    description: string;
    platformKind: TemplateDefinition["platformKind"];
    category: string | null;
    commandPattern: string;
    params: TemplateParamDefinition[];
  }): void;
  (event: "clone-template", payload: { templateId: string; newName: string }): void;
  (event: "delete-template", templateId: string): void;
  (event: "create-new"): void;
  (event: "cancel-edit"): void;
}>();

const { t } = useI18n();

const blankDraft = (): TemplateDraft => ({
  name: "",
  description: "",
  platformKind: "linux-shell",
  category: "",
  commandPattern: "",
  params: [],
});

const draft = reactive<TemplateDraft>(blankDraft());
const cloneName = ref("");

const resetDraft = (template: TemplateDefinition | null): void => {
  Object.assign(draft, blankDraft());
  if (!template || template.builtIn) {
    cloneName.value = template ? `${template.name} Copy` : "";
    return;
  }

  draft.templateId = template.id;
  draft.name = template.name;
  draft.description = template.description;
  draft.platformKind = template.platformKind;
  draft.category = template.category ?? "";
  draft.commandPattern = template.commandPattern;
  draft.params = template.params.map((param) => ({
    ...param,
    defaultValue: param.defaultValue ?? "",
    optionsInput: param.options.join(", "),
  }));
};

watch(
  () => props.template,
  (template) => {
    resetDraft(template);
  },
  { immediate: true }
);

const headerLabel = computed(() => {
  if (!props.template) return t("templates.newTemplate");
  return props.template.builtIn ? t("templates.cloneBuiltin") : t("templates.editTemplate");
});

const saveLabel = computed(() => (draft.templateId ? t("templates.save") : t("templates.create")));

const addParam = (): void => {
  draft.params.push({
    id: `tpl_param_${Math.random().toString(36).slice(2, 8)}`,
    paramKey: "",
    label: "",
    type: "text",
    required: false,
    defaultValue: "",
    options: [],
    optionsInput: "",
  });
};

const removeParam = (index: number): void => {
  draft.params.splice(index, 1);
};

const emitSave = (): void => {
  emit("save-template", {
    templateId: draft.templateId,
    name: draft.name,
    description: draft.description,
    platformKind: draft.platformKind,
    category: draft.category.trim() || null,
    commandPattern: draft.commandPattern,
    params: draft.params.map(({ optionsInput, defaultValue, ...param }) => ({
      ...param,
      defaultValue: defaultValue || null,
      options: param.type === "single-select"
        ? optionsInput.split(",").map((option) => option.trim()).filter(Boolean)
        : [],
    })),
  });
};

const emitClone = (): void => {
  if (!props.template) return;
  emit("clone-template", { templateId: props.template.id, newName: cloneName.value || `${props.template.name} Copy` });
};
</script>

<style scoped>
.template-editor {
  border: 1px solid #2e435f;
  border-radius: 12px;
  background: rgba(20, 38, 60, 0.45);
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.header-row,
.footer-actions,
.header-actions {
  display: flex;
  justify-content: space-between;
  gap: 8px;
}

.header-row {
  align-items: center;
}

.header-row h3,
.params-block h4 {
  margin: 0;
}

label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
}

input,
textarea,
select,
button {
  border: 1px solid #2f3f54;
  background: #101722;
  color: #d3e8ff;
  border-radius: 6px;
  padding: 8px;
}

textarea {
  min-height: 72px;
}

.grid-two,
.grid-three {
  display: grid;
  gap: 8px;
}

.grid-two {
  grid-template-columns: 1fr 1fr;
}

.grid-three {
  grid-template-columns: 1fr 1fr 1fr;
}

.checkbox {
  justify-content: flex-end;
  align-items: center;
}

.params-block,
.param-card {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.param-card {
  border: 1px solid #2f3f54;
  border-radius: 8px;
  padding: 10px;
  background: #0f1620;
}

.readonly-note {
  margin: 0;
  color: #a5bed8;
  font-size: 12px;
}

.danger {
  border-color: #74414c;
  background: #2a1319;
}
</style>