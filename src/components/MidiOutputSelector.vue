<script setup lang="ts">
import { computed } from 'vue';
import { useMidi } from '../hooks/use-midi';
import Select from './common/Select.vue';
import IconButton from './common/IconButton.vue';

const { midi, scanOutput, connectOutput, disconnectOutput } = useMidi();
const selectedOutput = computed<string>({
    get: () => midi.value.outputConnection?.port.id || "",
    set: (val: string) => {
        if (val.length > 0) {
            connectOutput(val);
        }
    }
});

function handleRefresh() {
    scanOutput();
}

function handleDisconnect() {
    disconnectOutput();
}

</script>

<template>
    <div class="flex flex-row">
        <IconButton v-if="!selectedOutput" icon="refresh" title="Refresh Output Ports" class="p-2"
            @click="handleRefresh()">
        </IconButton>
        <IconButton v-else icon="power_off" title="Disconnect Output" class="p-2" @click="handleDisconnect()">
        </IconButton>
        <Select v-model="selectedOutput" :disabled="!!selectedOutput" label="Select MIDI Output"
            :title="selectedOutput ? `Output connected to ${midi.outputConnection?.port.name}` : undefined"
            :options="midi.availableOutputPorts.map(({ id, name }) => ({ value: id, label: name }))"></Select>
    </div>
</template>