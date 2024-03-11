<script setup>
import { ref, watch, onBeforeMount } from 'vue'
import { ElTree } from 'element-plus'
import { invoke } from "@tauri-apps/api/tauri";

onBeforeMount(() => {
    load_config()
})

const filterText = ref('')
const treeRef = ref('');
const data = ref([]);

const defaultProps = {
    children: 'children',
    label: 'label',
}

watch(filterText, (val) => {
    treeRef.value?.filter(val)
})

function filterNode(value, data) {
    if (!value) return true
    return data.label.includes(value)
}


async function load_config() {
    await invoke("load_config", {})
        .then((message) => {
            const parsedData = JSON.parse(message);
            var array = parsedData.map(item => {
                return {
                    id: item.id,
                    label: item.nickname,
                    is_load: false,
                    children: []
                }
            });
            console.log('tarui: ', array)
            data.value = array;
        }).catch((error) => console.error(error));
}

function reload() {
    let ndata = data.value[0]
    let newChild = { id: Math.floor(Math.random() * 100), label: 'testtest', children: [] }

    ndata.children.push(newChild)
    data.value = [...data.value]
}

</script>

<template>
    <el-scrollbar>
        <el-input v-model="filterText" style="width: 240px" placeholder="过滤" />

        <el-tree ref="treeRef" style="max-width: 600px" class="filter-tree" :props="defaultProps" :data="data"
            :filter-node-method="filterNode" node-key="id" />

        <el-button @click="load(true)">Load</el-button>
    </el-scrollbar>
</template>



<style scoped>
.el-header {
    position: relative;
    background-color: var(--el-color-primary-light-7);
    color: var(--el-text-color-primary);
}

.el-aside {
    color: var(--el-text-color-primary);
    background: var(--el-color-primary-light-8);
}

.el-menu {
    border-right: none;
}

.el-main {
    padding: 0;
}

.toolbar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    right: 20px;
}
</style>
