<template>
    <el-tabs v-model="editableTabsValue" type="card" class="demo-tabs" editable @edit="handleTabsEdit">
        <template #add-icon>
            <span>+</span>
        </template>
        <el-tab-pane v-for="item in editableTabs" :key="item.name" :label="item.title" :name="item.name">
            {{ item.content }}
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import type { TabPaneName } from 'element-plus'

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
.el-tabs__content {
    padding: 32px;
    color: #6b778c;
    font-size: 32px;
    font-weight: 600;
}

.el-tabs__new-tab {
    margin: 10px 10px 10px 10px;
    float: left;
}

.el-tabs__item {
    user-select: none;
    -webkit-user-select: none;
}
</style>
