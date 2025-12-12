<script setup lang="ts">
import { ref } from 'vue';
import { useLogin } from '@/composables/useLogin';

import Button from '@/components/ui/button/Button.vue';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import ChatLive from '@/components/icons/ChatLive.vue';
import { useRouter } from 'vue-router';

const router = useRouter();

//composables
const { login, validateUsername } = useLogin();

//variables
const username = ref<string>('');
const errors = ref<null | string>(null);

const handleLogin = () => {
    errors.value = validateUsername(username.value);
    if (errors.value === null) {
        login(username.value);
        router.push('/');
    }
};

const handleKeyPress = (event: KeyboardEvent) => {
    if (event.key === 'Enter') {
        handleLogin();
    }
};
</script>

<template>
    <div class="min-h-screen bg-linear-to-br from-background via-muted/20 to-background flex items-center justify-center p-4">
        <!-- Logo/Brand -->
        <ChatLive />

        <!-- Login Card -->
        <div class="w-full max-w-md">
            <div class="bg-card border border-border rounded-xl shadow-lg p-8 space-y-6">
                <!-- Header -->
                <div class="text-center space-y-2">
                    <h1 class="text-3xl font-bold text-foreground">Bienvenido</h1>
                    <p class="text-muted-foreground">Ingresa tu nombre de usuario para comenzar a chatear</p>
                </div>

                <!-- Form -->
                <div class="space-y-4">
                    <div class="space-y-2">
                        <Label for="username" class="text-sm font-medium">Nombre de usuario</Label>
                        <Input 
                            id="username"
                            v-model="username"
                            type="text" 
                            placeholder="Ej: juan_perez" 
                            class="w-full"
                            @keypress="handleKeyPress"
                            :class="{ 'border-destructive focus-visible:ring-destructive': errors }"
                        />
                        <p v-if="errors" class="text-sm text-destructive flex items-center gap-1">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="12" y1="8" x2="12" y2="12"/>
                                <line x1="12" y1="16" x2="12.01" y2="16"/>
                            </svg>
                            {{ errors }}
                        </p>
                    </div>

                    <Button 
                        @click="handleLogin" 
                        class="w-full"
                        size="lg"
                    >
                        Ingresar al chat
                    </Button>
                </div>

                <!-- Footer -->
                <div class="pt-4 border-t border-border">
                    <p class="text-xs text-center text-muted-foreground">
                        Desarrollado por Joan Garfias
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.bg-card {
    animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
</style>