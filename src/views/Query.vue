<template>
    <div>
        <code-mirror ref="cm" v-model="input" :dark="dark" basic />
    </div>
</template>

<script setup>
import { ref, type Ref } from 'vue'

import CodeMirror from 'vue-codemirror6';

const cm: Ref<InstanceType<typeof CodeMirror> | undefined> = ref();

const input: Ref<string> = ref(`# The quick brown fox jumps over the lazy dog.`);

defineProps({ dark: Boolean });

</script>

<style scoped>
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
