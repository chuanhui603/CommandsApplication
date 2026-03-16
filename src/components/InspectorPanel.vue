<template>
  <div class="inspector">
    <div class="header-row">
      <h3>{{ t("inspector.title") }}</h3>
      <button v-if="node" type="button" class="danger" @click="emit('delete-node', node.id)">
        {{ t("inspector.deleteNode") }}
      </button>
    </div>
    <template v-if="node">
      <label>
        {{ t("inspector.nodeTitle") }}
        <input :value="node.title" @input="onChange('title', $event)" />
      </label>
      <label>
        {{ t("inspector.templateId") }}
        <input :value="node.templateId ?? ''" @input="onChange('templateId', $event)" />
      </label>
      <div v-if="template" class="template-summary">
        <strong>{{ template.name }}</strong>
        <p>{{ template.description }}</p>
        <code>{{ template.commandPattern }}</code>
      </div>
      <label>
        {{ t("inspector.notes") }}
        <textarea :value="node.notes" @input="onChange('notes', $event)" />
      </label>
      <label class="checkbox">
        <input
          type="checkbox"
          :checked="node.includeInOutput"
          @change="onToggleInclude($event)"
        />
        {{ t("inspector.includeInOutput") }}
      </label>
      <label class="checkbox">
        <input
          type="checkbox"
          :checked="isActivePath"
          @change="emit('set-active-path', node.id)"
        />
        {{ t("inspector.setActivePath") }}
      </label>
      <div class="params">
        <div class="params-header">
          <h4>{{ t("inspector.parameters") }}</h4>
          <button type="button" @click="addParam">{{ t("inspector.addParam") }}</button>
        </div>
        <ul v-if="template?.params.length" class="param-hints">
          <li v-for="definition in template.params" :key="definition.id">
            <strong>{{ definition.label || definition.paramKey }}</strong>
            <span>{{ definition.paramKey }}</span>
            <span v-if="definition.required">{{ t("inspector.required") }}</span>
            <span v-if="definition.defaultValue">{{ t("inspector.default") }}: {{ definition.defaultValue }}</span>
          </li>
        </ul>
        <div v-for="(param, idx) in node.params" :key="param.id" class="param-row">
          <input :value="param.paramKey" :placeholder="String(t('inspector.paramKey'))" @input="onParamChange(idx, 'paramKey', $event)" />
          <select :value="param.paramType" @change="onParamChange(idx, 'paramType', $event)">
            <option value="text">text</option>
            <option value="boolean">boolean</option>
            <option value="single-select">single-select</option>
            <option value="path">path</option>
          </select>
          <input :value="param.value" :placeholder="String(t('inspector.paramValue'))" @input="onParamChange(idx, 'value', $event)" />
          <button type="button" @click="removeParam(idx)">x</button>
        </div>
      </div>
    </template>
    <p v-else>{{ t("inspector.empty") }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { CommandNode, NodeParamValue, TemplateDefinition } from "../domain/models";
import { useI18n } from "../i18n";

const { t } = useI18n();

const props = defineProps<{
  node: CommandNode | null;
  activePathId: string | null;
  template: TemplateDefinition | null;
}>();

const emit = defineEmits<{
  (event: "update-node", payload: { id: string; patch: Partial<CommandNode> }): void;
  (event: "set-active-path", nodeId: string): void;
  (event: "delete-node", nodeId: string): void;
}>();

const isActivePath = computed(() => props.node?.id === props.activePathId);

const onChange = (field: "title" | "templateId" | "notes", event: Event): void => {
  if (!props.node) return;
  const value = (event.target as HTMLInputElement | HTMLTextAreaElement).value;
  emit("update-node", {
    id: props.node.id,
    patch: { [field]: field === "templateId" ? value || null : value } as Partial<CommandNode>
  });
};

const onToggleInclude = (event: Event): void => {
  if (!props.node) return;
  emit("update-node", {
    id: props.node.id,
    patch: { includeInOutput: (event.target as HTMLInputElement).checked }
  });
};

const addParam = (): void => {
  if (!props.node) return;
  const next: NodeParamValue[] = [
    ...props.node.params,
    {
      id: `param_${Math.random().toString(36).slice(2, 8)}`,
      paramKey: "",
      paramType: "text",
      value: ""
    }
  ];
  emit("update-node", { id: props.node.id, patch: { params: next } });
};

const removeParam = (idx: number): void => {
  if (!props.node) return;
  emit("update-node", {
    id: props.node.id,
    patch: { params: props.node.params.filter((_, index) => index !== idx) }
  });
};

const onParamChange = (
  idx: number,
  field: "paramKey" | "paramType" | "value",
  event: Event
): void => {
  if (!props.node) return;
  const value = (event.target as HTMLInputElement | HTMLSelectElement).value;
  const next = props.node.params.map((param, index) =>
    index === idx ? { ...param, [field]: value } : param
  );
  emit("update-node", { id: props.node.id, patch: { params: next } });
};
</script>

<style scoped>
.params {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.template-summary {
  border: 1px solid #2f3f54;
  border-radius: 8px;
  background: #0f1620;
  padding: 10px;
}

.template-summary p {
  margin: 6px 0;
  color: #a5bed8;
  font-size: 12px;
}

.template-summary code {
  display: block;
  white-space: pre-wrap;
  word-break: break-word;
  color: #d3e8ff;
}

.params-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.params h4 {
  margin: 0;
  font-size: 12px;
}

.param-hints {
  margin: 0;
  padding-left: 18px;
  color: #a5bed8;
  font-size: 12px;
}

.param-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr auto;
  gap: 6px;
}

.inspector {
  display: flex;
  flex-direction: column;
  gap: 8px;
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

.danger {
  border-color: #74414c;
  background: #2a1319;
}

textarea {
  min-height: 88px;
}

.checkbox {
  flex-direction: row;
  align-items: center;
  gap: 8px;
}
</style>
