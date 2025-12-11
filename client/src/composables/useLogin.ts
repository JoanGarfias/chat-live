import { useAuthStore } from "@/stores/auth";
import {
    stringNotEmpty,
    stringMinLength,
    stringMaxLength,
    stringEqualsLength,
    isValidUsernameLength
} from "@/utils/utils";

export function useLogin() {
    const auth = useAuthStore();
  
    const validateUsername = (username: string): null | string => {
        if (stringEqualsLength(username, 0))
            return "El nombre de usuario no puede estar vacío.";
        else if (!stringMinLength(username, 3))
            return "El nombre de usuario debe tener al menos 3 caracteres.";
        else if (!isValidUsernameLength(username))
            return "El nombre de usuario solo puede contener letras y números, y debe tener entre 3 y 16 caracteres.";

        return null;
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