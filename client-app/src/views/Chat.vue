<script setup lang="ts">
//vue utils
import { computed, nextTick, onMounted, ref, watch } from 'vue';

//composables and types
import { useChatStore } from '@/stores/chat';
import { Message } from '@/types/Message';
import { useSocket } from "@/composables/useSocket";

const { send, connect } = useSocket();
const chatStore = useChatStore();

//components and icons
import ChatActions from '@/components/chat/ChatActions.vue';
import ConfigPanel from '@/components/config/ConfigPanel.vue';
import { CircleUserRound, Settings } from 'lucide-vue-next';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import ClearChatModal from '@/components/config/ClearChatModal.vue';

const messages = computed(() => chatStore.messages);
const isConfigOpen = ref<boolean>(false);
const isClearChatModalOpen = ref<boolean>(false);
const messagesContainer = ref<HTMLElement | null>(null);

const stateClass = computed(() => {
    console.log('Estado del socket:', chatStore.getState);
    switch (chatStore.getState) {
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

const receiveMessage = (msg: Message) => {
    chatStore.addMessage(msg);
    send(JSON.stringify(msg));
}

const scrollToBottom = () => {
    nextTick(() => {
        if (messagesContainer.value) {
            messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
    });
};

// Autoscroll cuando cambian los mensajes
watch(messages, () => {
    scrollToBottom();
}, { deep: true });

onMounted(async () => {
    console.log('Conectando con el socket...');
    await connect();
    scrollToBottom();
});


</script>

<template>
    <ConfigPanel v-model:open="isConfigOpen" />
    <ClearChatModal v-model:open="isClearChatModalOpen" />

    <div class="h-screen bg-linear-to-br from-background via-muted/20 to-background flex flex-col">
        <!-- Chat Container -->
        <div class="flex-1 flex flex-col bg-card overflow-hidden">
            <!-- Chat Header -->
            <div class="bg-accent/50 border-b border-border p-4 flex items-center justify-between">
                <div class="flex flex-col gap-2">
                    <!--<h2 class="text-xl font-semibold text-foreground">Chat Live</h2> -->
                    <div class="flex flex-row gap-2 items-center">
                        <CircleUserRound class="h-6 w-6 text-muted-foreground" />
                        <p class="text-md text-muted-foreground">{{ chatStore.getName }}</p>
                    </div>
                </div>
                <div class="flex flex-row items-center gap-4">
                    <span :class="stateClass" class="rounded-full w-3 h-3"></span>
                    <!-- Settings -->
                    <div class="items-center">
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <Settings class="h-5 w-5 text-muted-foreground" />
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>

                                <DropdownMenuLabel>Ajustes</DropdownMenuLabel>
                                <DropdownMenuSeparator />

                                <DropdownMenuItem @click="isConfigOpen = true">
                                    <span>IP/puerto</span>
                                </DropdownMenuItem>
                                <DropdownMenuItem @click="isClearChatModalOpen = true">
                                    <span>Limpiar chat</span>
                                </DropdownMenuItem>

                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                </div>
            </div>

            <!-- Chat Messages -->
            <div ref="messagesContainer" class="flex-1 p-4 overflow-y-auto space-y-4">
                <!-- Aquí irán los mensajes del chat -->
                <div v-for="(msg, index) in messages" :key="index" class="p-3 rounded-md" :class="msg.autor === chatStore.getName ? 'bg-blue-100 self-end' : 'bg-gray-100 self-start'">
                    <p class="font-semibold text-sm mb-1">{{ msg.autor }}</p>
                    <p class="text-md">{{ msg.mensaje }}</p>
                </div>
                <p v-if="chatStore.isChatCleaned && messages.length === 0" class="text-center text-muted-foreground">El chat ha sido limpiado.</p>
                <p v-else-if="messages.length === 0" class="text-center text-muted-foreground">No hay mensajes aún. ¡Empieza la conversación!</p>
            </div>

            <!-- Chat Actions -->
            <ChatActions @sendMessage="receiveMessage" />
        </div>
    </div>
</template>