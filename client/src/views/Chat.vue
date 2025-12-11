<script setup lang="ts">
import { computed, ref } from 'vue';

import { useChat } from '@/composables/useChat';

import ChatActions from '@/components/chat/ChatActions.vue';
import ConfigPanel from '@/components/config/ConfigPanel.vue';
import { CircleUserRound, Settings } from 'lucide-vue-next';
import Button from '@/components/ui/button/Button.vue';

const { getState, getName } = useChat();
const isConfigOpen = ref<boolean>(false);

const stateClass = computed(() => {
    switch (getState()) {
        case 'connected':
            return 'bg-emerald-500';
        case 'connecting':
            return 'bg-yellow-500';
        case 'disconnected':
            return 'bg-red-500';
        default:
            return 'bg-gray-500';
    }
});


</script>

<template>
    <ConfigPanel v-model:open="isConfigOpen" />

    <div class="h-screen bg-linear-to-br from-background via-muted/20 to-background flex flex-col">
        <!-- Chat Container -->
        <div class="flex-1 flex flex-col bg-card overflow-hidden">
            <!-- Chat Header -->
            <div class="bg-accent/50 border-b border-border p-4 flex items-center justify-between">
                <div class="flex flex-col gap-2">
                    <!--<h2 class="text-xl font-semibold text-foreground">Chat Live</h2> -->
                    <div class="flex flex-row gap-2 items-center">
                        <CircleUserRound class="h-6 w-6 text-muted-foreground" />
                        <p class="text-md text-muted-foreground">{{ getName() }}</p>
                    </div>
                </div>
                <div class="flex flex-row items-center gap-4">
                    <span :class="stateClass" class="rounded-full w-3 h-3"></span>
                    <!-- Settings -->
                    <div class="items-center">
                        <Button
                        variant="ghost"
                        @click="isConfigOpen = true"
                        class="flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition cursor-pointer">
                            <Settings class="h-5 w-5" />
                        </Button>
                    </div>
                </div>
            </div>

            <!-- Chat Messages -->
            <div class="flex-1 p-4 overflow-y-auto space-y-4">
                <!-- Aquí irán los mensajes del chat -->
                <p class="text-muted-foreground">No hay mensajes aún. Sé el primero en enviar uno.</p>
            </div>

            <!-- Chat Actions -->
            <ChatActions />
        </div>
    </div>
</template>

<style scoped>

</style>