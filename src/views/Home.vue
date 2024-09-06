<script setup>
import Top from '@@/views/Top.vue'
import LinkView from "./LinkView.vue";
import BaseView from "./BaseView.vue";
import LogView from "./LogView.vue";
</script>

<template>
  <Top/>
  <div class="card">
    <LinkView />
    <BaseView />
  </div>
  <LogView />
  <div class="result">
    <div style="padding-right: 1rem">链接</div>
    <input type="text" class="form-item">
    <button class="button button-fix result-button">生成订阅</button>
    <button class="button button-fix result-button">复制结果</button>
  </div>
</template>

<style scoped>

</style>
