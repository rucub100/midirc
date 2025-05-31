<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

const model = defineModel<string>({ required: true });
defineProps<{
    options: Array<{ value: string; label: string }>;
}>();
const isMenuOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

function update(value: string) {
    model.value = value;
    isMenuOpen.value = false;
}

function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}

function handleClickOutside(event: MouseEvent) {
    if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
        isMenuOpen.value = false;
    }
}

onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});


onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
    <div class="flex w-[200px] border-b relative" ref="dropdownRef">
        <button class="flex flex-row w-full cursor-pointer" @click="toggleMenu">
            <div class="mr-auto overflow-hidden text-ellipsis whitespace-nowrap">
                {{options.find(option => option.value === model)?.label || 'None'}}
            </div>
            <span class="material-symbols-sharp">expand_more</span>
        </button>
        <menu v-if="isMenuOpen" class="flex flex-col w-max absolute top-[calc(100%+10px)] left-0 border select-none">
            <li class="px-2 py-1 cursor-not-allowed">None</li>
            <li v-for="option in options" :key="option.value"
                class="px-2 py-1 cursor-pointer hover:bg-[var(--color-hover)]"
                :class="{ 'bg-[var(--color-active)]': model === option.value }" @click="update(option.value)">{{
                    option.label }}</li>
        </menu>
    </div>
</template>
