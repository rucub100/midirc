<script setup lang="ts">
import { ref } from 'vue';
import { useMidi } from '../hooks/use-midi';

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
            <button class="flex items-center justify-center cursor-pointer p-1 hover:bg-[rgba(127,127,127,0.1)]" @click="refresh()">
                <span class="material-symbols-sharp">refresh</span>
            </button>
            <label class="mr-2">Input:</label>
            <select v-model="selectedInput">
                <option value="">--Please select input--</option>
                <template v-for="input in midi.availableInputPorts">
                    <option :value="input.id">{{ input.name }}</option>
                </template>
            </select>
            <label class="mr-2">Output:</label>
            <select v-model="selectedOutput">
                <option value="">--Please select output--</option>
                <template v-for="output in midi.availableOutputPorts">
                    <option :value="output.id">{{ output.name }}</option>
                </template>
            </select>
        </div>
    </div>
</template>