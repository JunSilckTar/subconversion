<template>
  <div class="top-container">
    <p>Sub-Conversion</p>
    <div style="display: flex">
      <a href="https://github.com/" class="item github">
        <img src="../assets/icons/github.svg" class="logo" alt="Github logo"/>
        Frontend
      </a>
      <a href="https://github.com/tindy2013/subconverter" class="item github">
        <img src="../assets/icons/github.svg" class="logo" alt="Github logo"/>
        Backend
      </a>
      <a href="https://vitejs.dev " class="item vite " >
        <img src="../assets/icons/vite.svg" class="logo" alt="Vite logo"/> Vite
      </a>
      <a href="https://vuejs.org/" class="item vue">
        <img src="../assets/icons/vue.svg" class="logo" alt="Vue logo"/>
        Vue
      </a>
    </div>
  </div>
</template>

<style>
.top-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 1rem 2rem 0;
}

.logo {
  width: 1rem;
  padding-right: 0.5rem;
}

.item {
  display: flex;
  padding: 0.4rem 1em;
  font-size: 1.04rem;
  transition: all 400ms ease-in-out;
}

.item:hover {
  transform: scale(1.1);
}
.github:hover{
  filter: drop-shadow(0 0 5px #2c3e50);
}
.vite:hover{
  filter: drop-shadow(0 0 5px #ab45f6);
}
.vue:hover{
  filter: drop-shadow(0 0 5px #64b687);
}
</style>
<script setup lang="ts">
</script>