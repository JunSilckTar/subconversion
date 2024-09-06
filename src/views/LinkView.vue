<script setup>
import {onMounted, ref} from "vue";
import {useComponentStore} from "../plugins/pinia/Store.js";
import {usePinia} from "../plugins/pinia/usePinia.js";

// 使用 usePinia 封装 store
const pinia = usePinia();

// 显示, 控制历史链接的展开
const linkIsVisible = ref(false);

// 历史链接
function toggleLink() {
  linkIsVisible.value = !linkIsVisible.value;
}

// 显示, 控制更多设置的展开
const formIsVisible = ref(false);

// 更多功能
function toggleForm() {
  formIsVisible.value = !formIsVisible.value;
  pinia.update('formIsVisible', formIsVisible.value);
  console.log(pinia.query('formIsVisible'));
}

onMounted(()=>{
  pinia.add('formIsVisible', formIsVisible.value);
})

</script>

<template>
  <div class="form form-textarea">
    <div class="form-label">订阅链接</div>
    <textarea class="form-item" style="height: 6rem"
              placeholder="需要转换的链接, 支持ss、ssr、Vmess, 链接之间请用 | 分割"></textarea>
    <div class="button-group">
      <button :class="['button', { 'button-fix': linkIsVisible }]"
              @click="toggleLink()">历史链接
      </button>
      <button :class="['button', { 'button-fix': formIsVisible }]"
              @click="toggleForm">更多设置
      </button>
    </div>
  </div>
  <transition name="fade">
    <div v-show="linkIsVisible" class="card ">

    </div>
  </transition>
</template>


<style>
.button-group {
  display: flex;
  height: 6rem;
  flex-direction: column;
  justify-content: space-between;
  align-content: center;
}

.button {

}
</style>
