import { useAuthStore } from "@/stores/auth";

export function useChat(){

    const auth = useAuthStore();

    const getState = () => {
        return "connected";
    }

    const getName = () => {
        return auth.user ? auth.user.nombre_usuario : "Invitado";
    }

    return {
        getState,
        getName
    }
}