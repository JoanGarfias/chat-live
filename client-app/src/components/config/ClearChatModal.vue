<script setup lang="ts">
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useChatStore } from '@/stores/chat';

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  'update:open': [value: boolean],
  'config-saved': []
}>();

const chatStore = useChatStore();

const handleOpenChange = (value: boolean) => {
  emit('update:open', value);
}

const handleSubmit = (event: Event) => {
    event.preventDefault();
    chatStore.clearChat();
    emit('update:open', false);
}

</script>

<template>
  <Dialog :open="props.open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[285px]">
      <form @submit="handleSubmit">
        <DialogHeader>
          <DialogTitle>Limpieza de chat</DialogTitle>
          <DialogDescription>
            Confirma la limpieza del chat.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter class="mt-4 flex flex-row justify-center ">
        <Button type="submit">
            Limpiar chat
        </Button>  
        <DialogClose as-child>
            <Button type="button" variant="outline">
              Cancelar
            </Button>
          </DialogClose>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>