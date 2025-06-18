<script setup lang="ts">
import { PropType } from 'vue';

defineProps({
    icon: {
        type: String as PropType<string>,
        required: true,
    },
    active: {
        type: Boolean as PropType<boolean>,
        default: false,
    },
    disabled: {
        type: Boolean as PropType<boolean>,
        default: false,
    },
});
</script>

<template>
    <button class="flex items-center justify-center cursor-pointer hover:text-[var(--color-text)]" :disabled="disabled"
        :class="{ 'text-[var(--color-text)]/75': !active, 'text-[var(--color-text)]': active }">
        <span class="material-symbols-sharp">{{ icon }}</span>
    </button>
</template>