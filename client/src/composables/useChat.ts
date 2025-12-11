import { useAuthStore } from "@/stores/auth";
import type { Message } from "@/types/Message";
import { ref } from "vue";

export function useChat(){

    const auth = useAuthStore();
    
    const messages = ref<Message[]>([]);

    const getState = () => {
        return "connected";
    }

    const getName = () => {
        return auth.user ? auth.user.nombre_usuario : "Invitado";
    }

    const addMessage = (msg: Message) => {
        messages.value.push(msg);
    }

    const clearChat = () => {
        messages.value = [];
    }

    return {
        getState,
        getName,
        addMessage,
        clearChat
    }
}