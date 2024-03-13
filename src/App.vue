<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import DataBasePanel from './components/DataBasePanel.vue';
import MainPanel from './components/MainPanel.vue';
</script>

<template>
  <el-container>
    <el-aside width="30%">
      <DataBasePanel />
    </el-aside>
    <el-main>
      <MainPanel />
    </el-main>
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

.el-aside {
  max-width: 300px;
}

.el-main {
  padding: 0;
  margin-left: 10px;
  margin-right: 20px;
}
</style>
