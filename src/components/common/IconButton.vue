<script setup lang="ts">
defineProps({
    icon: {
        type: String,
        required: true,
    }
});
</script>

<template>
    <button
        class="flex items-center justify-center cursor-pointer hover:bg-[var(--color-hover)] active:bg-[var(--color-active)]">
        <span class="material-symbols-sharp">{{ icon }}</span>
    </button>
</template>