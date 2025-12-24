//vue utils
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

//pinia stores
import { useConfigStore } from "@/stores/config";
import { useChatStore } from "@/stores/chat";

//tauri utils
import { listen } from "@tauri-apps/api/event";

//types
import type { Configuration } from "@/types/Configuration";
import type { Message } from "@/types/Message";

export function useSocket() {
  const isConnected = ref(false);
  const msgServer = ref<string[]>([]);
  const serverConfig = ref<Configuration | null>(null);

  const chatStore = useChatStore();

  const connect = async () => {
    const configStore = useConfigStore();
    const config = configStore.getConfig;
    if (!config) {
      console.error("No hay configuración disponible");
      return;
    }

    try {
      await invoke("connect_socket", { 
        serverInfo: {
          ip: config.ip, 
          port: config.port
        }
      });
      // Guardar la configuración usada para la conexión
      serverConfig.value = { ip: config.ip, port: config.port };
      isConnected.value = true;

      await listen<string>("socket_message", (event) => {
        msgServer.value.push(event.payload);
        console.log("Mensaje del servidor:", event.payload);
      });

      listen("socket_message", (event : any) => {
        msgServer.value.push(event.payload);
        console.log("Mensaje del servidor:", event.payload);

                //Conectando mensaje con el chat UI
        const msgPayload = event.payload;
        const message: Message = {
          autor: "Usuario",
          mensaje: msgPayload,
        };
        chatStore.addMessage(message);
      });

    } catch (error) {
      console.error("Error al conectar:", error);
      isConnected.value = false;
    }
  };

  const send = async (msg: string) => {
    try {
      await invoke("send_message", { msg: msg });
      console.log("Mensaje enviado:", msg);
    } catch (error) {
      console.error("Error al enviar mensaje:", error);
    }
  };

  const disconnect = () => {
    if (!serverConfig.value) {
      console.warn("No hay conexión activa para desconectar");
      isConnected.value = false;
      msgServer.value = [];
      return;
    }

    invoke("disconnect_socket",
      {
        ip: serverConfig.value.ip,
        port: serverConfig.value.port
      }
    ).then(() => {
      console.log("Desconectado correctamente");
      isConnected.value = false;
      msgServer.value = [];
      serverConfig.value = null;
    }).catch((error) => {
      console.error("Error al desconectar:", error);
    });
  };

  return { isConnected, msgServer, send, connect, disconnect };
}
