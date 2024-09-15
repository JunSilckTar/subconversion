
<script setup>

import { invoke } from '@tauri-apps/api';

function openInBrowser(url){
  invoke('open_url',{url})
      .then(()=>{
        console.log('URL opened in default browser'+ url)
      })
      .catch((error) => {
        console.error('Failed to open URL:', error);
      });
}

</script>

<template>
  <div class="top-container">
    <div class="title-area">
      <img class="icon" src="@@/assets/icons/icon_life.png" alt="icon">
      <p style="padding-left: 0.8rem">Sub-Conversion</p>
    </div>
    <div style="display: flex">
      <a class="item github" @click="openInBrowser('https://github.com/JunSilckTar/subconversion')">
        <img src="@@/assets/icons/github.svg" class="logo" alt="Github logo"/>
        Frontend
      </a>
      <a class="item github" @click="openInBrowser('https://github.com/tindy2013/subconverter')" >
        <img src="@@/assets/icons/github.svg" class="logo" alt="Github logo"/>
        Backend
      </a>
      <a class="item vite " @click="openInBrowser('https://vitejs.dev')" >
        <img src="@@/assets/icons/vite.svg" class="logo" alt="Vite logo"/> Vite
      </a>
      <a class="item vue" @click="openInBrowser('https://vuejs.org/')">
        <img src="@@/assets/icons/vue.svg" class="logo" alt="Vue logo"/>
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

.title-area {
  display: flex;
  align-items: center;
}

.icon {
  width: 1.6rem;
  padding-left: 0.2rem;
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

.github:hover {
  filter: drop-shadow(0 0 5px #2c3e50);
}

.vite:hover {
  filter: drop-shadow(0 0 5px #ab45f6);
}

.vue:hover {
  filter: drop-shadow(0 0 5px #64b687);
}
</style>
