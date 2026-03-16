<template>
  <div class="inspector">
    <h3>Inspector</h3>
    <template v-if="node">
      <label>
        Title
        <input :value="node.title" @input="onChange('title', $event)" />
      </label>
      <label>
        Template ID
        <input :value="node.templateId ?? ''" @input="onChange('templateId', $event)" />
      </label>
      <label>
        Notes
        <textarea :value="node.notes" @input="onChange('notes', $event)" />
      </label>
      <label class="checkbox">
        <input
          type="checkbox"
          :checked="node.includeInOutput"
          @change="onToggleInclude($event)"
        />
        Include in output
      </label>
      <label class="checkbox">
        <input
          type="checkbox"
          :checked="isActivePath"
          @change="emit('set-active-path', node.id)"
        />
        Set as active path
      </label>
      <div class="params">
        <div class="params-header">
          <h4>Parameters</h4>
          <button type="button" @click="addParam">+ Add</button>
        </div>
        <div v-for="(param, idx) in node.params" :key="param.id" class="param-row">
          <input :value="param.paramKey" placeholder="key" @input="onParamChange(idx, 'paramKey', $event)" />
          <select :value="param.paramType" @change="onParamChange(idx, 'paramType', $event)">
            <option value="text">text</option>
            <option value="boolean">boolean</option>
            <option value="single-select">single-select</option>
            <option value="path">path</option>
          </select>
          <input :value="param.value" placeholder="value" @input="onParamChange(idx, 'value', $event)" />
          <button type="button" @click="removeParam(idx)">x</button>
        </div>
      </div>
    </template>
    <p v-else>Select a node to edit details.</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { CommandNode, NodeParamValue } from "../domain/models";

const props = defineProps<{
  node: CommandNode | null;
  activePathId: string | null;
}>();

const emit = defineEmits<{
  (event: "update-node", payload: { id: string; patch: Partial<CommandNode> }): void;
  (event: "set-active-path", nodeId: string): void;
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

.params-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.params h4 {
  margin: 0;
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
select {
  border: 1px solid #2f3f54;
  background: #101722;
  color: #d3e8ff;
  border-radius: 6px;
  padding: 8px;
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
