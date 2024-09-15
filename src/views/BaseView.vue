
<script setup >
import {ref} from "vue";
import VisibleFormView from "./VisibleFormView.vue";

// 默认关闭, 控制选项卡的弹窗
const clientDropdown = ref(false);

// 下拉选项
const clientOptions = ref(['Clash', 'ClashR', 'Surge3', 'Surge4', 'Quantumult',
  'QuantumultX', 'Surfboard', 'Loon', 'Mellow', 'V2Ray', 'Trojan', 'SSR', 'SS', 'SS_Android', 'Surge2']);
// 客户端数据, 初始化为第一项
const clientSelected = ref(clientOptions.value[0]);

// 触发client窗口
function clientToggle() {
  clientDropdown.value = !clientDropdown.value;
}

// 选择后触发
function selectOption(option) {
  clientSelected.value = option;
  clientDropdown.value = false;  // 选中后关闭下拉列表
}

</script>

<template>
  <div class="form" style="position: relative;">
    <div class="label-fix">客户端</div>
    <input type="text"
           readonly
           style="flex-grow:0;min-width: 8.8rem"
           :value="clientSelected"
           @click="clientToggle">
    <div class="label-fix padding-fix">后端地址</div>
    <input type="text" placeholder="可自定义地址">
    <ul v-show="clientDropdown" class="dropdown-options">
      <li v-for="option in clientOptions"
          :key="option" @click="selectOption(option)">{{ option }}
      </li>
    </ul>
  </div>
  <VisibleFormView />
</template>