<script setup>
import { ref, watch, onBeforeMount } from 'vue'
import { ElTree } from 'element-plus'
import { invoke } from "@tauri-apps/api/tauri";
import { v4 as uuidv4 } from 'uuid';

onBeforeMount(() => {
    load_config()
})

const filterText = ref('')
const treeRef = ref('');
const data = ref([]);

const defaultProps = {
    id: 'id',
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
                    q_type: 1,
                    db: null,
                    uid: "" + item.id,
                    label: item.nickname,
                    config: item,
                    is_loaded: false,
                    children: []
                }
            });
            console.log('tarui: ', array)
            data.value = array;
        }).catch((error) => console.error(error));
}

async function specific_query(q_type, db, uid) {
    return await invoke("specific_query", { qType: q_type, db: db, uid: uid })
        .then((message) => {
            console.log('specific_query: ', message)
            const parsedData = JSON.parse(message);
            var array = parsedData[0].rows.map(item => {
                let id = uuidv4();
                return {
                    id: id,
                    uid: uid,
                    label: Object.values(item.columns)[0],
                    db: q_type === 1 ? item.columns.Database : db,
                    q_type: q_type + 1,
                    children: [],
                    is_loaded: q_type === 1 ? false : true
                }
            });
            console.log('tarui: ', array)
            return array;
        }).catch((error) => {
            console.error(error)
            return [];
        });
}

async function new_connection(data) {
    await invoke("new_conn", {
        uid: data.uid,
        id: data.config.id,
        nickname: data.config.nickname,
        host: data.config.host,
        port: data.config.port,
        user: data.config.user,
        password: data.config.password,
        sqlType: data.config.sql_type
    })
        .then((message) => {
            console.log('new_connection: ', message)
        }).catch((error) => console.error(error));
}

function renderContent(h, { node, data, store }) {
    return h('span', {
        onDblclick: () => handleNodeDblclick(node, data),
        style: {
            userSelect: 'none',
            width: '100%',
            textAlign: 'left'
        }
    }, node.label);

}

function handleNodeDblclick(node, data) {
    console.log(data);

    if (data.is_loaded) {
        if (data.q_type === 3) {

        }
        return;
    }
    if (data.q_type === 1) {
        data.db = null;
        new_connection(data).then(() => {
            specific_query(data.q_type, data.db, data.uid).then((children) => {
                data.children = children;
                data.is_loaded = true;
            })
        })
    } else if (data.q_type === 2) {
        specific_query(data.q_type, data.db, data.uid).then((children) => {
            data.children = children;
            data.is_loaded = true;
        })
    }
}


</script>

<template>
    <el-scrollbar>
        <el-input v-model="filterText" style="width: 240px" placeholder="过滤" />

        <el-tree ref="treeRef" style="max-width: 600px" class="filter-tree" :props="defaultProps" :data="data"
            :render-content="renderContent" :filter-node-method="filterNode" node-key="id" />

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
