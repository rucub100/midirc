<script setup lang="ts">
import { PropType } from 'vue';
import IconButton from '../common/IconButton.vue';

defineProps({
    index: {
        type: Number as PropType<number>,
        required: true,
    },
});

const emit = defineEmits<{
    (e: 'play'): void
}>();
</script>

<template>
    <div class="flex flex-row w-max items-center border border-[var(--color-outline)] rounded p-2">
        <IconButton icon="play_arrow" class="p-2" @click="emit('play')"></IconButton>
        <h2>Recording #{{ index + 1 }}</h2>
        <IconButton icon="file_save" class="p-2"></IconButton>
        <IconButton icon="delete" class="p-2"></IconButton>
    </div>
</template>