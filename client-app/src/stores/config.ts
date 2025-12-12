import { defineStore } from "pinia";
import type { Configuration } from '@/types/Configuration';

export const useConfigStore = defineStore('config', {
  state: () => ({
    config: {
        ip: '192.168.1.1',
        port: 5000,
    } as Configuration | null,
  }),
  getters: {
    getConfig: (state) => state.config,
  },
  actions: {
    setConfig(config: Configuration) {
      this.config = config;
      localStorage.setItem('appConfig', JSON.stringify(config));
    },

    loadConfig() : Configuration | null {
      const stored = localStorage.getItem('appConfig');
      if (stored) {
        try {
            this.config = JSON.parse(stored) as Configuration;
            return this.config;
        } catch (error) {
            console.error('Error al cargar la configuración:', error);
            this.config = null;
            return null;
        }
      }
      return null;
    },

    clearConfig() {
      this.config = null;
      localStorage.removeItem('appConfig');
    }
  },
});