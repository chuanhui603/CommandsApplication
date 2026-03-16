<template>
  <div class="tree">
    <button class="add-button" type="button" @click="$emit('add-node')">{{ t("tree.addNode") }}</button>
    <div
      class="toolbox-item"
      draggable="true"
      @dragstart="onDragStart"
    >
      {{ t("tree.dragNode") }}
    </div>
    <p v-if="nodes.length === 0" class="empty-state">{{ t("tree.empty") }}</p>
    <ul v-else>
      <li
        v-for="node in nodes"
        :key="node.id"
        :class="{ selected: node.id === selectedNodeId }"
        @click="$emit('select-node', node.id)"
      >
        {{ node.title }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import type { CommandNode } from "../domain/models";
import { useI18n } from "../i18n";

const { t } = useI18n();

defineProps<{
  nodes: CommandNode[];
  selectedNodeId: string | null;
}>();

defineEmits<{
  (event: "add-node"): void;
  (event: "select-node", nodeId: string): void;
}>();

const onDragStart = (event: DragEvent): void => {
  event.dataTransfer?.setData("application/x-node-template", "command-node");
};
</script>

<style scoped>
.tree {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.add-button {
  border: 1px solid #29434e;
  border-radius: 6px;
  background: #0f2027;
  color: #d3e8ff;
  padding: 8px;
  cursor: pointer;
}

.toolbox-item {
  border: 1px dashed #3f5b80;
  border-radius: 6px;
  padding: 8px;
  color: #a5bed8;
  cursor: grab;
}

.empty-state {
  margin: 0;
  color: #a5bed8;
  font-size: 13px;
}

ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

li {
  border-radius: 6px;
  padding: 8px;
  background: #151f2c;
  cursor: pointer;
}

li.selected {
  outline: 1px solid #90caf9;
}
</style>
