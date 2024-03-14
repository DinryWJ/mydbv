<!-- src/views/About.vue -->
<template>
    <div>
        <h1>About Page</h1>
        <p>Learn more about us.</p>
    </div>
</template>


<script setup>
// 在 `<script setup>` 中可以直接使用变量、响应式数据和组件选项，无需再写 `export default`
import { ref } from 'vue';

const name = ref('About');
</script>