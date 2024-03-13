<!-- ref https://logue.dev/vue-codemirror6/ -->
<template>
    <el-tabs v-model="editableTabsValue" type="card" editable @edit="handleTabsEdit">
        <el-tab-pane v-for="item in editableTabs" :key="item.name" :label="item.title" :name="item.name">
            <code-mirror ref="cm" v-model="input" :dark="dark" basic />
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { ref, type Ref } from 'vue'
import type { TabPaneName } from 'element-plus'

import CodeMirror from 'vue-codemirror6';

/** CodeMirror Instance */
const cm: Ref<InstanceType<typeof CodeMirror> | undefined> = ref();

/** Demo text */
const input: Ref<string> = ref(`# The quick brown fox jumps over the lazy dog.

[Lorem ipsum](https://www.lipsum.com/) dolor sit amet, **consectetur** adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.`);

// Sync dark mode
defineProps({ dark: Boolean });

let tabIndex = 0
const editableTabsValue = ref('0')
const editableTabs = ref<{
    title: string;
    name: string;
    content: string,
    type: string,
}[]>([])

const handleTabsEdit = (
    targetName: TabPaneName | undefined,
    action: 'remove' | 'add'
) => {
    if (action === 'add') {
        const newTabName = `${++tabIndex}`
        editableTabs.value.push({
            title: 'New Tab',
            type: 'query',
            name: newTabName,
            content: 'New Tab content',
        })
        editableTabsValue.value = newTabName
    } else if (action === 'remove') {
        const tabs = editableTabs.value
        let activeName = editableTabsValue.value
        if (activeName === targetName) {
            tabs.forEach((tab, index) => {
                if (tab.name === targetName) {
                    const nextTab = tabs[index + 1] || tabs[index - 1]
                    if (nextTab) {
                        activeName = nextTab.name
                    }
                }
            })
        }

        editableTabsValue.value = activeName
        editableTabs.value = tabs.filter((tab) => tab.name !== targetName)
    }
}
</script>

<style>
.el-tabs {
    height: 100%;
}

.el-tab-pane {
    height: 100%;
}

.el-tabs__content {
    color: #6b778c;
    height: 100%;
}

.el-tabs__new-tab {
    margin: 10px 10px 10px 10px;
    float: left;
}

.el-tabs__item {
    user-select: none;
    -webkit-user-select: none;
}

.vue-codemirror {
    height: 100%;
}

.cm-editor {
    height: 100%;
}

.cm-line {
    text-align: left;
}

.cm-gutters {
    background-color: transparent !important;
}
</style>
