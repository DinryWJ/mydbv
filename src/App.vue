<script lang="ts" setup>
import { ref } from 'vue'
import DataBasePanel from './components/DataBasePanel.vue';
import MainPanel from './components/MainPanel.vue';

const asideWidth = ref(300);
let startX = 0;
let dragging = false;


const startDrag = (event) => {
  startX = event.clientX;
  dragging = true;
  document.addEventListener('mousemove', handleDrag);
  document.addEventListener('mouseup', stopDrag);
  event.preventDefault(); // 阻止默认的拖动行为

}


const handleDrag = (event) => {
  if (dragging) {
    const diffX = event.clientX - startX;
    asideWidth.value += diffX;
    startX = event.clientX;
  }
}

const stopDrag = () => {
  dragging = false;
  document.removeEventListener('mousemove', handleDrag);
  document.removeEventListener('mouseup', stopDrag);
}

</script>

<template>
  <el-container>
    <el-aside :style="{ width: asideWidth + 'px' }">
      <DataBasePanel />
    </el-aside>
    <el-main>
      <div class="handle" :style="{ left: asideWidth + 'px' }" @mousedown="startDrag"></div>
      <MainPanel />
    </el-main>
  </el-container>
</template>

<style>
body {
  margin: 0;
  position: relative;
  overflow: hidden;
}

.el-container {
  height: 100%;
  width: 100%;
  overflow-y: auto;
}

.el-aside {
  position: relative;
  overflow: hidden;
}
</style>
<style scoped>
#app {
  height: 100vh;
  width: 100vw;
  position: fixed;
  top: 0;
  bottom: 0;
  overflow: hidden;
}


.handle {
  position: absolute;
  top: 50%;
  width: 5px;
  height: 100%;
  transition: background-color 0.3s ease;
  /* 添加过渡效果 */
  z-index: 50;
  transform: translate(-50%, -50%);
  background-color: #f4f4f4;
  cursor: col-resize;
}

.handle:hover {
  background-color: #ccc;
}

.el-main {
  padding: 0;
}
</style>
