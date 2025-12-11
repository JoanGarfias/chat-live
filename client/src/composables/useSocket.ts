import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useConfigStore } from "@/stores/config";

export function useSocket() {
  const configStore = useConfigStore();
  const isConnected = ref(false);
  const msgServer = ref<string[]>([]);

  const connect = async () => {
    const config = configStore.getConfig;
    if (!config) {
      console.error("No hay configuración disponible");
      return;
    }

    try {
      await invoke("connect_socket", { 
        ip: config.ip, 
        port: config.port 
      });
      isConnected.value = true;

      await listen<string>("server-message", (event) => {
        msgServer.value.push(event.payload);
      });
    } catch (error) {
      console.error("Error al conectar:", error);
      isConnected.value = false;
    }
  };

  const send = async (msg: string) => {
    try {
      await invoke("send_message", { message: msg });
    } catch (error) {
      console.error("Error al enviar mensaje:", error);
    }
  };

  const disconnect = () => {
    isConnected.value = false;
  };

  return { isConnected, msgServer, send, connect, disconnect };
}
