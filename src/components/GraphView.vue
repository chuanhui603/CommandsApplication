<template>
  <VueFlow
    class="graph-view"
    :nodes="flowNodes"
    :edges="flowEdges"
    fit-view-on-init
    @connect="onConnect"
    @node-drag-stop="onNodeDragStop"
  />
</template>

<script setup lang="ts">
import { computed } from "vue";
import { VueFlow, type Connection, type Edge, type Node, type NodeDragEvent } from "@vue-flow/core";
import type { CommandEdge, CommandNode, LayoutState } from "../domain/models";

const props = defineProps<{
  nodes: CommandNode[];
  edges: CommandEdge[];
  layouts: LayoutState;
}>();

const emit = defineEmits<{
  (event: "add-edge", edge: CommandEdge): void;
  (event: "node-moved", payload: { id: string; x: number; y: number }): void;
}>();

const flowNodes = computed<Node[]>(() =>
  props.nodes.map((node) => ({
    id: node.id,
    data: { label: node.title },
    position: props.layouts.graph[node.id] ?? { x: 80, y: 80 }
  }))
);

const flowEdges = computed<Edge[]>(() =>
  props.edges.map((edge) => ({
    id: edge.id,
    source: edge.sourceNodeId,
    target: edge.targetNodeId
  }))
);

const onConnect = (connection: Connection): void => {
  if (!connection.source || !connection.target) return;
  emit("add-edge", {
    id: `edge_${connection.source}_${connection.target}`,
    sourceNodeId: connection.source,
    targetNodeId: connection.target,
    edgeType: "flow",
    priority: null,
    enabled: true
  });
};

const onNodeDragStop = (event: NodeDragEvent): void => {
  emit("node-moved", {
    id: event.node.id,
    x: event.node.position.x,
    y: event.node.position.y
  });
};
</script>

<style scoped>
.graph-view {
  width: 100%;
  height: 100%;
}
</style>
