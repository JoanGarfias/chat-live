<script setup lang="ts">
import { ref } from 'vue';

import ChatActions from '@/components/chat/ChatActions.vue';
import ConfigPanel from '@/components/config/ConfigPanel.vue';
import { Settings } from 'lucide-vue-next';
import Button from '@/components/ui/button/Button.vue';

const isConfigOpen = ref<boolean>(false);

</script>

<template>
    <ConfigPanel v-model:open="isConfigOpen" />

    <div class="h-screen bg-linear-to-br from-background via-muted/20 to-background flex flex-col">
        <!-- Chat Container -->
        <div class="flex-1 flex flex-col bg-card overflow-hidden">
            <!-- Chat Header -->
            <div class="bg-accent/50 border-b border-border p-4 flex items-center justify-between">
                <h2 class="text-xl font-semibold text-foreground">Chat Live</h2>
                <div class="flex flex-row items-center gap-4">
                    <span class="bg-emerald-500 rounded-full w-3 h-3"></span>
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