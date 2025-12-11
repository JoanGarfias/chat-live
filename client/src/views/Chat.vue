<script setup lang="ts">
import ChatActions from '@/components/chat/ChatActions.vue';

</script>

<template>
    <div class="h-screen bg-linear-to-br from-background via-muted/20 to-background flex flex-col">
        <!-- Chat Container -->
        <div class="flex-1 flex flex-col bg-card overflow-hidden">
            <!-- Chat Header -->
            <div class="bg-accent/50 border-b border-border p-4 flex items-center justify-between">
                <h2 class="text-xl font-semibold text-foreground">Chat Live</h2>
                <span class="text-sm text-muted-foreground">Conectado</span>
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