import { defineStore } from "pinia";
import { useAuthStore } from "@/stores/auth";
import type { Message } from "@/types/Message";

export const useChatStore = defineStore('chat', {
  state: () => ({
    messages: [] as Message[],
    chatCleaned: false,
  }),
  
  getters: {
    getState: () => "connected",
    getName: () => {
      const auth = useAuthStore();
      return auth.user ? auth.user.nombre_usuario : "Invitado";
    }
  },
  
  actions: {
    addMessage(msg: Message) {
      this.messages.push(msg);
    },
    
    clearChat() {
      this.messages = [];
      this.chatCleaned = true;
    }
  }
});