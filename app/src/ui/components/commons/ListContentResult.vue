<script setup lang="ts">
import type { ListResult } from '~/stores/ListResult';

const props = defineProps<{
  result: ListResult<any>
  emptyStateMsg?: String
  errorStateMsg?: String
}>()
</script>

<template>
  <slot v-if="result.isSuccess()"/>
  <div v-if="result.isEmpty()" class="empty-state card">
    <p>{{ emptyStateMsg }}</p>
  </div>
  <div v-if="result.isError()" class="error-state card">
    <p>{{ errorStateMsg ? errorStateMsg : result.error.message }}</p>
  </div>
  <div class="loading-state">
    <Loading v-if="result.isLoading()" />
  </div>
</template>

<style scoped lang="scss">
@import "../../style/pages.scss";

.empty-state {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.error-state {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.loading-state {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}
</style>