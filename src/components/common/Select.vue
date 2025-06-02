<script setup lang="ts">
import { computed, onMounted, onUnmounted, PropType, ref } from 'vue';

const model = defineModel<string>({ required: true });
const props = defineProps({
    options: {
        type: Array as PropType<{ value: string; label: string }[]>,
        required: true,
    },
    label: {
        type: String as PropType<string>,
        default: '',
    },
    disabled: {
        type: Boolean as PropType<boolean>,
        default: false,
    },
});
const isMenuOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

const valueLabel = computed(() => {
    return props.options.find(option => option.value === model.value)?.label || 'None'
});

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
    <div class="flex w-[200px] border-b relative" :class="{ 'text-[var(--color-text-muted)]': disabled }"
        ref="dropdownRef">
        <button class="flex flex-row items-end w-full" :class="{ 'cursor-pointer': !disabled }" @click="toggleMenu"
            :disabled="disabled">
            <label v-if="label.length > 0" class="absolute text-xs top-0" :class="{ 'cursor-pointer': !disabled }">{{
                label }}</label>
            <div class="mr-auto overflow-hidden text-ellipsis whitespace-nowrap"
                :class="{ 'pt-4': label.length > 0, 'text-[var(--color-text-muted)]': !model }">
                {{ valueLabel }}
            </div>
            <span class="material-symbols-sharp">expand_more</span>
        </button>
        <menu v-if="isMenuOpen" class="flex flex-col w-max absolute top-[calc(100%+10px)] left-0 border select-none">
            <li
                class="px-2 py-1 cursor-not-allowed border-b border-[var(--color-outline)] text-[var(--color-text-muted)]">
                None</li>
            <li v-for="option in options" :key="option.value"
                class="px-2 py-1 cursor-pointer hover:bg-[var(--color-hover)]"
                :class="{ 'bg-[var(--color-active)]': model === option.value }" @click="update(option.value)">{{
                    option.label }}</li>
        </menu>
    </div>
</template>
