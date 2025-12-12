<script setup lang="ts">
import type { Configuration } from '@/types/Configuration'
import {  useConfigStore } from '@/stores/config'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { ref, onMounted } from 'vue'

const configStore = useConfigStore()

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  'update:open': [value: boolean],
  'config-saved': []
}>()

const handleOpenChange = (value: boolean) => {
  emit('update:open', value)
}

// Inicializar con los valores del store (por defecto o guardados)
const config = ref<Configuration>({
    ip: configStore.config?.ip || '',
    port: configStore.config?.port || 0
});

// Cargar configuración guardada al montar el componente
onMounted(() => {
  const savedConfig = configStore.loadConfig()
  if (savedConfig) {
    config.value.ip = savedConfig.ip
    config.value.port = savedConfig.port
  } else if (configStore.config) {
    config.value.ip = configStore.config.ip
    config.value.port = configStore.config.port
  }
});

const loading = ref<boolean>(false);
const errors = ref<string | null>(null);

const handleSubmit = (event: Event) => {
    event.preventDefault()
    loading.value = true
    if(config.value.ip.trim() === '' || config.value.port <= 0) {
        errors.value = 'Por favor, ingresa una IP y un puerto válidos.';
        loading.value = false;
        return;
    }
    errors.value = null;

    configStore.setConfig(config.value)
    emit('config-saved');
    setTimeout(() => {
        handleOpenChange(false);
        loading.value = false;
    }, 1000)
}

</script>

<template>
  <Dialog :open="props.open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[425px]">
      <form @submit="handleSubmit">
        <DialogHeader>
          <DialogTitle>Configuración del chat</DialogTitle>
          <DialogDescription>
            Configura la IP del servidor y el puerto.
          </DialogDescription>
        </DialogHeader>
        <div class="grid gap-4">
          <div class="grid gap-3">
            <Label for="server-ip">IP del servidor</Label>
            <Input id="server-ip" v-model="config.ip" name="server-ip" placeholder="192.168.1.1" />
          </div>
          <div class="grid gap-3">
            <Label for="server-port">Puerto</Label>
            <Input id="server-port" v-model.number="config.port" min="1" name="server-port" type="number" placeholder="5000" />
          </div>
          <p v-if="errors" class="text-sm text-destructive">{{ errors }}</p>
        </div>
        <DialogFooter class="mt-4">
          <DialogClose as-child>
            <Button type="button" variant="outline">
              Cancelar
            </Button>
          </DialogClose>
          <Button type="submit" :disabled="loading">
            {{ loading ? 'Guardando...' : 'Guardar' }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>