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
    <input type="text" class="">
    <button class="button-fix button-ani">生成订阅</button>
    <button class="button-fix button-ani">复制结果</button>
  </div>
</template>
