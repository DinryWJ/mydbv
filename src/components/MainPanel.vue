<template>
    <el-tabs v-model="editableTabsValue" type="card" editable @edit="handleTabsEdit">
        <el-tab-pane v-for="item in editableTabs" :key="item.name" :label="item.title" :name="item.name">
            <router-link :to="{ name: item.name }">{{ item.title }}</router-link>
        </el-tab-pane>
    </el-tabs>
</template>


<script lang="ts" setup>
import { ref, type Ref } from 'vue'
import type { TabPaneName } from 'element-plus'


let tabIndex = 2
const editableTabsValue = ref('2')
const editableTabs = ref<{
    name: string,
    type: string
}[]>([])

const handleTabsEdit = (
    targetName: TabPaneName | undefined,
    action: 'remove' | 'add'
) => {
    if (action === 'add') {
        const newTabName = `${++tabIndex}`
        editableTabs.value.push({
            title: 'Query',
            name: 'Query'
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
</style>
