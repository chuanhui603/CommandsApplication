<template>
  <p class="status" :data-status="status">Save status: {{ status }}</p>
</template>

<script setup lang="ts">
import type { SaveStatus } from "../domain/models";

defineProps<{ status: SaveStatus }>();
</script>

<style scoped>
.status {
  margin: 0;
  font-size: 12px;
  color: #9fb3c8;
}

.status[data-status="error"] {
  color: #ff9e9e;
}

.status[data-status="saved"] {
  color: #89d99d;
}
</style>
