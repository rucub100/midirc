<script setup lang="ts">
import IconButton from '../common/IconButton.vue';

</script>

<template>
    <div class="flex flex-row p-2 border border-[var(--color-outline)] w-max rounded">
        <IconButton icon="fiber_manual_record" class="p-2"></IconButton>
        <IconButton icon="play_arrow" class="p-2"></IconButton>
        <IconButton icon="pause" class="p-2"></IconButton>
        <IconButton icon="stop" class="p-2"></IconButton>
    </div>
</template>