import { createRouter, createMemoryHistory } from 'vue-router';
import { useAuthStore } from "@/stores/auth";

import Chat from '@/views/Chat.vue';
import Login from '@/views/Login.vue';

// Para aplicaciones de escritorio con Tauri, usamos createMemoryHistory
// en lugar de createWebHistory para evitar conflictos con el protocolo tauri://
const router = createRouter({
  history: createMemoryHistory(import.meta.env.BASE_URL),
  routes: [
        {
        path: '/login',
        name: 'Login',
        component: Login,
        meta: { 
            requiresAuth: false,
            layout: 'AuthLayout' 
        },
        },
        {
        path: '/',
        name: 'Home',
        component: Chat,
        meta: { 
            requiresAuth: true,
            layout: 'DefaultLayout'
        },
        },
    ]
})

// Guard de navegación para apps de escritorio
router.beforeEach((to : any, _from : any, next : any) => {
  const auth = useAuthStore();

  if (to.meta.requiresAuth && !auth.isLoggedIn) {
    // Redirigir a login si la ruta requiere autenticación
    next({ name: "Login" });
  }
  else if (to.path === '/login' && auth.isLoggedIn) {
    // Si ya está logueado y va al login, redirigir a home
    next({ name: "Home" });
  }
  else {
    next();
  }
});

export default router