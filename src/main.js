import { createApp } from "vue";
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from "./App.vue";
import router from './router';

const pinia = createPinia()

createApp(App).use(pinia).use(ElementPlus).use(router).mount("#app");
