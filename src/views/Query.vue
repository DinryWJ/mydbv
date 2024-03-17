<template>
    <div class="wrap">
        <code-mirror class="top" :style="{ height: topHeight + 'vh' }" ref="cm" v-model="input" :dark="dark" basic />
        <div class="resize-bar" @mousedown="startResize">
            <!-- Resize bar -->
        </div>
        <el-table class="bottom" :data="tableData" :style="{ height: bottomHeight + 'vh' }">
            <el-table-column prop="date" label="Date" width="180" />
            <el-table-column prop="name" label="Name" width="180" />
            <el-table-column prop="address" label="Address" />
        </el-table>
    </div>
</template>

<script lang="ts" setup>
import { ref, type Ref } from 'vue'

import CodeMirror from 'vue-codemirror6';

const cm: Ref<InstanceType<typeof CodeMirror> | undefined> = ref();

const input: Ref<string> = ref(`# The quick brown fox jumps over the lazy dog.`);

defineProps({ dark: Boolean });

const tableData = [
    {
        date: '2016-05-03',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-02',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-04',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-01',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-08',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-06',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
    {
        date: '2016-05-07',
        name: 'Tom',
        address: 'No. 189, Grove St, Los Angeles',
    },
]

const topHeight = ref(70);
const bottomHeight = ref(30);
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
.wrap {
    height: 100vh;
    display: flex;
    flex-direction: column;
}

.vue-codemirror {
    height: 100%;
}

.cm-editor {
    width: 100%;
    overflow-y: auto;
}

.cm-content {
    text-align: left;
}

.cm-gutters {
    background-color: transparent;
}

.top,
.bottom {
    display: flex;
}

.resize-bar {
    height: 10px;
    cursor: ns-resize;
    background-color: #ccc;
}
</style>
