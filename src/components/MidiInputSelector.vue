<script setup lang="ts">
import { computed } from 'vue';
import { useMidi } from '../hooks/use-midi';
import Select from './common/Select.vue';
import IconButton from './common/IconButton.vue';

const { midi, scanInput, connectInput, disconnectInput } = useMidi();
const selectedInput = computed<string>({
    get: () => midi.value.inputConnection?.id || "",
    set: (val: string) => {
        if (val.length > 0) {
            connectInput(val);
        }
    }
});

function handleRefresh() {
    scanInput();
}

function handleDisconnect() {
    disconnectInput();
}

</script>

<template>
    <div class="flex flex-row">
        <IconButton v-if="!selectedInput" icon="refresh" title="Refresh Input Ports" class="p-2"
            @click="handleRefresh()">
        </IconButton>
        <IconButton v-else icon="power_off" title="Disconnect Input" class="p-2" @click="handleDisconnect()">
        </IconButton>
        <Select v-model="selectedInput" :disabled="!!selectedInput" label="Select MIDI Input"
            :title="selectedInput ? `Input connected to ${midi.inputConnection?.name}` : undefined"
            :options="midi.availableInputPorts.map(({ id, name }) => ({ value: id, label: name }))"></Select>
    </div>
</template>