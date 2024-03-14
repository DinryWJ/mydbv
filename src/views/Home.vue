<!-- src/views/Home.vue -->
<template>
    <div>
        <h1>Home Page</h1>
        <p>{{ message }}</p>
    </div>
</template>

<script setup>
// 在 `<script setup>` 中可以直接使用变量、响应式数据和组件选项，无需再写 `export default`
import { ref } from 'vue';

const message = ref('Welcome to my website!');
</script>
