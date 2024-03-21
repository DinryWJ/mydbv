<template>
    <div class="wrap">
        <div class="query-options">
            <el-select class="top-select" v-model="selectConn" placeholder="choose connection" size="small">
                <el-option filterable v-for="item in activeConnList" :key="item.id" :label="item.label"
                    :value="item.label" />
            </el-select>
            <el-select class="top-select" v-model="selectDb" placeholder="choose db" size="small">
                <el-option filterable v-for="item in activeDbList" :key="item.value" :label="item.label"
                    :value="item.value" />
            </el-select>
        </div>
        <code-mirror class="top" :style="{ height: `calc(${topHeight}vh - 70px)` }" ref="cm" v-model="input"
            :dark="dark" basic />
        <div class="resize-bar" @mousedown="startResize">
            <!-- Resize bar -->
        </div>
        <el-table :data="tableData" style="width: 100%">
            <el-table-column v-for="column in tableColumns" :key="column.prop" :prop="column.prop" :label="column.label"
                :width="column.width">
                <template #default="scope">
                    <el-input class="no-border" v-model="scope.row[column.prop]"></el-input>
                </template>
            </el-table-column>
        </el-table>
    </div>
</template>

<script lang="ts" setup>
import { ref, type Ref, onMounted } from 'vue'
import CodeMirror from 'vue-codemirror6';
import { useActivateDatabaseStore } from '../stores/activateDatabase';

const cm: Ref<InstanceType<typeof CodeMirror> | undefined> = ref();

const input: Ref<string> = ref(`# select * from mysql;`);

defineProps({ dark: Boolean });


const tableData = ref([
    {
        id: 1,
        date: '2020-05-02',
        name: 'John Smith',
        address: 'No.1518,  Jinshajiang Road, Putuo District'
    },
    {
        id: 1,
        date: '2020-05-02',
        name: 'John Smith',
        address: 'No.1518,  Jinshajiang Road, Putuo District'
    },
    // ...更多数据
])

interface TableColumn {
    prop: string;
    label: string;
    width: string;
}

let tableColumns = ref<TableColumn[]>([]);

const generateTableColumns = () => {
    if (tableData.value.length > 0) {
        const firstRow = tableData.value[0];
        tableColumns.value = Object.keys(firstRow).map(key => {
            return { prop: key, label: key.charAt(0).toUpperCase() + key.slice(1), width: '180px' };
        });
    }
};

onMounted(generateTableColumns); // Call generateTableColumns on component mount

const selectConn = ref('')
const activateDatabase = useActivateDatabaseStore();
const activeConnList = activateDatabase.getAll();

const selectDb = ref('')
const activeDbList = [
    {
        value: 'Option1',
        label: 'Option1',
    },
    {
        value: 'Option2',
        label: 'Option2',
    },
]

const topHeight = ref(68);
const bottomHeight = ref(35);
let isResizing = false;
let startY = 0;

const startResize = (event) => {
    isResizing = true;
    startY = event.clientY;
    document.addEventListener('mousemove', handleResize);
    document.addEventListener('mouseup', stopResize);
};

const handleResize = (event) => {
    if (isResizing) {
        const deltaY = event.clientY - startY;
        topHeight.value += deltaY / window.innerHeight * 100;
        bottomHeight.value -= deltaY / window.innerHeight * 100;
        startY = event.clientY; // Update start position
    }
};

const stopResize = () => {
    isResizing = false;
    document.removeEventListener('mousemove', handleResize);
    document.removeEventListener('mouseup', stopResize);
};
</script>

<style>
.cm-editor {
    width: 100% !important;
    overflow-y: auto;
}

.cm-content {
    text-align: left;
}

.cm-gutters {
    background-color: transparent;
}
</style>
<style scoped>
.no-border :deep(.el-input__wrapper) {
    background-color: #FFFFFF;
    box-shadow: 0 0 0 0;
}

.wrap {
    height: 100vh;
    display: flex;
    flex-direction: column;
}

.vue-codemirror {
    height: 100%;
}

.top,
.bottom {
    display: flex;
}

.resize-bar {
    height: 4px;
    cursor: ns-resize;
    background-color: #ccc;
}

.query-options {
    display: flex;
    margin-bottom: 10px;
}

.top-select {
    width: 120px;
    margin-right: 20px;
}
</style>
