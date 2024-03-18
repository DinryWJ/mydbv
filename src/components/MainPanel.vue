<template>
    <el-tabs style="width: 100%;" v-model="editableTabsValue" type="card" editable @edit="handleTabsEdit">
        <el-tab-pane v-for="item in editableTabs" :label="item.name" :name="item.name">
            <component :is="item.component" :key="item.name"></component>
        </el-tab-pane>
    </el-tabs>
</template>


<script lang="ts" setup>
import { ref, type Ref } from 'vue'
import type { TabPaneName } from 'element-plus'
import Query from '../views/Query.vue';

let tabIndex = 0
const editableTabsValue = ref('0')
const editableTabs = ref<{
    name: string,
    component: any
}[]>([])

const handleTabsEdit = (
    targetName: TabPaneName | undefined,
    action: 'remove' | 'add'
) => {
    if (action === 'add') {
        const newTabName = `${++tabIndex}`
        let name = 'Query' + newTabName;
        editableTabs.value.push({
            name: name,
            component: Query
        })
        editableTabsValue.value = name
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
.el-tabs--card {
    height: calc(100vh);
    overflow: hidden;
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
