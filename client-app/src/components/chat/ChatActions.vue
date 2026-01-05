<script setup lang="ts">
//vue utils
import { ref } from 'vue';

//components and icons
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Message } from '@/types/Message';
import { Send } from 'lucide-vue-next';

// composables
import { useChat } from '@/composables/useChat';
const { getName } = useChat();


const messageToSend = ref<Message>({
    autor: '',
    mensaje: ''
});

const textareaRef = ref<HTMLTextAreaElement | null>(null);

const emit = defineEmits<{
    (e: 'sendMessage', msg: Message): void
}>();

const sendMessage = () => {
    const newMessage : Message = { ...messageToSend.value, autor: getName() };
    emit('sendMessage', newMessage);
    messageToSend.value.mensaje = '';
    if(textareaRef.value){
        textareaRef.value.scrollTop = textareaRef.value.scrollHeight;
    }
};

</script>

<template>
    <div class="bg-accent/50 border-t border-border p-4 flex items-center gap-4">
        <Textarea
            ref="textareaRef"
            placeholder="Escribe tu mensaje..."
            class="flex-1 resize-none h-10 min-h-10 max-h-10 border border-input bg-background rounded-md px-3 overflow-hidden"
            rows="1"
            v-model="messageToSend.mensaje"
            @keydown.enter.prevent="sendMessage"
        />
        <Button variant="outline" class="rounded-full h-12 w-12 flex items-center justify-center p-0" @click="sendMessage">
            <Send class="h-6 w-6" />
        </Button>
    </div>
</template>

<style scoped>
/* Ocultar scrollbar en el textarea */
textarea {
    scrollbar-width: none; /* Firefox */
    -ms-overflow-style: none; /* IE and Edge */
}

textarea::-webkit-scrollbar {
    display: none; /* Chrome, Safari y Opera */
}
</style>