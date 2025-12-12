import { defineStore } from "pinia";
import type { User } from '@/types/User';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null as User | null,
    lastLogin: null as number | null,
  }),
  getters: {
    getUser: (state) => state.user,
    isLoggedIn: (state) => state.user !== null,
  },
  actions: {
    checkUserIsLoggedIn() {
      return this.user !== null;
    },

    setUser(user: User) {
      this.user = user;
      this.lastLogin = Date.now();
    },

    logout() {
      this.user = null;
      this.lastLogin = null;
    }
  },
});