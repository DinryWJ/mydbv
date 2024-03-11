<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import DataBasePanel from './components/DataBasePanel.vue';
</script>

<template>
  <el-container>
    <el-aside width="200px">
      <DataBasePanel />
    </el-aside>
    <el-main>Main</el-main>
  </el-container>
</template>

<style>
body {
  margin: 0;
  position: relative;
  display: flex;
  overflow: hidden;
  place-items: center;
}

#app {
  text-align: center;
  height: 100vh;
  width: 100vw;
  position: fixed;
  top: 0;
  bottom: 0;
  overflow: hidden;
}

.el-container {
  height: 100%;
  width: 100%;
  overflow-y: auto;
}
</style>
