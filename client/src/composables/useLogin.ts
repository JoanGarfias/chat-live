import { useAuthStore } from "@/stores/auth";
const auth = useAuthStore();

export function useLogin() {
  
    const validateUsername = (username: string): null | string => {
        if (username.trim().length >= 3) {
            return null;
        } else if (username.trim().length === 0) {
            return "El nombre de usuario no puede estar vacío.";
        }
        else if (username.trim().length < 3) {
            return "El nombre de usuario debe tener al menos 3 caracteres.";
        }
        return "Nombre de usuario inválido.";
    }

    const login = (username: string) => {
        auth.setUser({ nombre_usuario: username });
        console.log(`Usuario ${username} ha iniciado sesión.`);
    }

    return {
        login,
        validateUsername
    }
}