<template>
  <div class="top-container">
    <p>Sub-Conversion</p>
    <div class="top-item">
      <a href="https://github.com/" class="item github">
        <img src="../assets/github.svg" class="logo" alt="Github logo"/>
        Frontend
      </a>
      <a href="https://github.com/tindy2013/subconverter" class="item github">
        <img src="../assets/github.svg" class="logo" alt="Github logo"/>
        Backend
      </a>
      <a href="https://vitejs.dev " class="item vite " >
        <img src="../assets/vite.svg" class="logo" alt="Vite logo"/> Vite
      </a>
      <a href="https://vuejs.org/" class="item vue">
        <img src="../assets/vue.svg" class="logo" alt="Vue logo"/>
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
  font-family: inherit;
  width: 100vw;
  padding-top: 1rem;
  height: auto;
  color: #2c3e50;
  text-align: center;
}

p{
  padding-left:2rem;
  line-height: 0;
  font-weight: 800;
  font-size: 1.4rem;
}

.top-item{
  display: flex;
  padding-right: 2rem;
}



a {
  text-decoration: none;
  color: inherit;
  font-size: 1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  outline: none;
}

.logo {
  height: 1.2rem;
  padding-right: 0.5rem;
}

.item {
  padding: 0.4rem 1em;
  transition: all 400ms ease-in-out;
}

.item:hover {
  transform: scale(1.1);
}
.github:hover{
  filter: drop-shadow(0 0 5px #000000);
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