<script setup lang="ts">
import { ref } from 'vue';
import { useMidi } from '../hooks/use-midi';
import Select from '../components/common/Select.vue';
import Button from '../components/common/Button.vue';
import IconButton from '../components/common/IconButton.vue';

const { midi, scan, connect, disconnect } = useMidi();
const selectedInput = ref<string>("");
const selectedOutput = ref<string>("");

function refresh() {
    selectedInput.value = "";
    selectedOutput.value = "";
    scan();
}

function handleConnect() {
    connect(selectedInput.value, selectedOutput.value);
}
</script>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row items-center p-4 gap-4">
            <IconButton icon="refresh" class="p-2" @click="refresh()"></IconButton>
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
            <Button @click="handleConnect">Connect</Button>
        </div>
    </div>
</template>