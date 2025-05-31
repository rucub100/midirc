<script setup lang="ts">
import { ref } from 'vue';
import { useMidi } from '../hooks/use-midi';
import Select from '../components/common/Select.vue';
import Button from '../components/common/Button.vue';

const { midi, scan } = useMidi();
const selectedInput = ref<string>("");
const selectedOutput = ref<string>("");

function refresh() {
    selectedInput.value = "";
    selectedOutput.value = "";
    scan();
}
</script>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row p-4 gap-4">
            <button class="flex items-center justify-center cursor-pointer p-1 hover:bg-[rgba(127,127,127,0.1)]"
                @click="refresh()">
                <span class="material-symbols-sharp">refresh</span>
            </button>
            <div class="flex flex-col">
                <label class="mb-2 text-sm">Input:</label>
                <Select v-model="selectedInput"
                    :options="midi.availableInputPorts.map(({ id, name }) => ({ value: id, label: name }))"></Select>
            </div>
            <div class="flex flex-col">
                <label class="mb-2 text-sm">Output:</label>
                <Select v-model="selectedOutput"
                    :options="midi.availableOutputPorts.map(({ id, name }) => ({ value: id, label: name }))"></Select>
            </div>
            <Button>Connect</Button>
        </div>
    </div>
</template>