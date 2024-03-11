<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const db = ref("");
const name = ref("");

const id = 0;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("load_config", { name: name.value })
    .then((message) => {
      console.log(message);
      greetMsg.value = message;
    })
    .catch((error) => console.error(error));
}
async function greet2() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("connect_db", { id: 1, nickname: db.value, host: "172.16.13.47", port: 3307, user: "root", password: "root", password: "123456", database: "ai_web", sqlType: "mysql" })
    .then((message) => {
      console.log(message);
      greetMsg.value = message;
    })
    .catch((error) => console.error(error));
}
async function greet3() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("ddl", {
    id: id,
    db: "ai_web",
    table: "d_alarm_info"})
    .then((message) => {
      console.log(message);
      greetMsg.value = message;
    })
    .catch((error) => console.error(error));
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>
  <br />
  <form class="row2" @submit.prevent="greet2">
    <input id="greet-input2" v-model="db" placeholder="Enter a name..." />
    <button type="submit">Greet2</button>
  </form>
  <br />
  <form class="row3" @submit.prevent="greet3">
    <input id="greet-input3" v-model="id" placeholder="Enter a name..." />
    <button type="submit">Greet3</button>
  </form>
  <p>{{ greetMsg }}</p>
</template>
