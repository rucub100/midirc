<script setup lang="ts">
import { useNavigation, View } from '../hooks/use-navigation';
import IconButton from './common/IconButton.vue';

const { currentView } = useNavigation();

const emit = defineEmits<{
    (e: 'navigate', view: View): void;
}>();
</script>

<template>
    <div class="flex flex-col h-full">
        <IconButton icon="piano" title="MIDI Setup" class="p-3" :active="currentView === 'setup'"
            @click="emit('navigate', 'setup')"></IconButton>
        <IconButton icon="voicemail_2" title="MIDI Recorder" class="p-3" :active="currentView === 'recorder'"
            @click="emit('navigate', 'recorder')">
        </IconButton>
        <!-- <IconButton icon="edit_audio" title="MIDI Editor" class="p-3" :active="currentView === 'editor'"
            @click="emit('navigate', 'editor')"></IconButton> -->
        <!-- <IconButton icon="play_lesson" title="Lessons" class="p-3">
        </IconButton> -->
    </div>
</template>