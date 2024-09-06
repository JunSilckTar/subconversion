import {createApp} from 'vue'
import {createPinia} from 'pinia';
import persisted from 'pinia-plugin-persistedstate';
import '@@/css/index.css'
import '@@/css/home.css'
import Home from '@@/views/Home.vue'


export const pinia = createPinia();
pinia.use(persisted);

export const app = createApp(Home);
app.use(pinia);
app.mount('#app');


