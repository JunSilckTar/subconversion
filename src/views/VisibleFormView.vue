
<script setup>
import {ref} from "vue";

// 使用Store
// 显示, 控制更多设置的展开
const formIsVisible = ref(false);

const checkGroup = ref([]);

</script>

<template>
  <transition name="fade">
    <div v-show="formIsVisible">
      <div class="form">
        <div class="form-label">文件名</div>
        <input type="text" class="form-item" placeholder="自定义文件名">
        <div class="form-label" style="padding-left: 1rem">包含匹配</div>
        <textarea class="form-item form-textarea" placeholder="节点名称包含的名字、支持正则"></textarea>
        <div class="form-label" style="padding-left: 1rem">排除匹配</div>
        <textarea class="form-item form-textarea" placeholder="节点名称不包含的名字、支持正则"></textarea>
      </div>
      <div class="form">
        <div class="form-label">远程配置</div>
        <input type="text" class="form-item">
      </div>
      <div class="check-box">
        <div class="check-items">
          <input style="margin-left: 0" type="checkbox" id="emoji" value="Emoji" v-model="checkGroup"/>
          <label for="emoji">Emoji</label>
          <input type="checkbox" id="clash" value="Clash New Field" v-model="checkGroup"/>
          <label for="clash">Clash New Field</label>
          <input type="checkbox" id="udp" value="启用UDP" v-model="checkGroup"/>
          <label for="udp">启用UDP</label>
        </div>
        <div class="check-items">
          <input type="checkbox" id="netease" value="网易云" v-model="checkGroup"/>
          <label for="netease">网易云</label>
          <input type="checkbox" id="surge" value="Surge.DoH" v-model="checkGroup"/>
          <label for="surge">Surge.DoH</label>
          <input type="checkbox" id="clashDoh" value="Clash.DoH" v-model="checkGroup"/>
          <label for="clashDoh">Clash.DoH</label>
        </div>
        <div class="check-items">
          <input type="checkbox" id="nodeType" value="节点类型" v-model="checkGroup"/>
          <label for="nodeType">节点类型</label>
          <input type="checkbox" id="sortNode" value="排序节点" v-model="checkGroup"/>
          <label for="sortNode">排序节点</label>
          <input type="checkbox" id="filterNode" value="过滤非法节点" v-model="checkGroup"/>
          <label for="filterNode">过滤非法节点</label>
        </div>
      </div>
    </div>
  </transition>
</template>