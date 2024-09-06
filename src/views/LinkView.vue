<script setup>
import {onMounted, ref} from "vue";
import {useComponentStore} from "../plugins/pinia/Store.js";
import {usePinia} from "../plugins/pinia/usePinia.js";

// 使用 usePinia 封装 store
const pinia = usePinia(useComponentStore());
// 检索值
const query = pinia.query('formIsVisible');

// 显示, 控制更多设置的展开 不要给默认值
let formIsVisible = ref()

onMounted(() => {
  // 初始化数据
  if (query) {
    // 检索值存在
    formIsVisible.value = query;
  } else {
    // 检索值不存在
    formIsVisible.value = false;
  }
})

// 更多功能
function toggleForm() {
  formIsVisible.value = !formIsVisible.value;
  pinia.update('formIsVisible', formIsVisible.value);
}

// 显示, 控制历史链接的展开
const linkIsVisible = ref(false);

// 历史链接
function toggleLink() {
  linkIsVisible.value = !linkIsVisible.value;
}


</script>

<template>
  <div class="form" style="align-items: start">
    <div class="label-fix" style="padding-top: 0.4rem">订阅链接</div>
    <textarea style="min-height: 6rem"
              placeholder="需要转换的链接, 支持ss、ssr、Vmess, 链接之间请用 | 分割"></textarea>
    <div class="button-group">
      <button :class="{'button-fix': formIsVisible }"
              @click="toggleForm">更多设置
      </button>
      <button :class="{'button-fix': linkIsVisible }"
              @click="toggleLink()">历史链接
      </button>
    </div>
  </div>
  <transition name="fade">
    <div v-show="linkIsVisible" class="card">

    </div>
  </transition>
</template>


<style>
.button-group {
  display: flex;
  height: 6rem;
  flex-direction: column;
  justify-content: space-between;
}
</style>
