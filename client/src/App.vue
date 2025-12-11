<script setup lang="ts">
import { ref } from "vue";

import Button from "@/components/ui/button/Button.vue";
import TextArea from "@/components/ui/textarea/TextArea.vue";

const message = ref("");
const count = ref(0);

const incrementCount = () => {
  count.value++;
};

</script>

<template>
  <div class="min-h-screen bg-background p-8">
    <div class="max-w-2xl mx-auto space-y-6">
      <div class="space-y-2">
        <h1 class="text-3xl font-bold text-foreground">Prueba de shadcn-vue</h1>
        <p class="text-muted-foreground">Verificando que los componentes funcionen correctamente</p>
      </div>
      
      <div class="space-y-4 p-6 border rounded-lg bg-card">
        <h2 class="text-xl font-semibold">Botones</h2>
        <div class="flex flex-wrap gap-3">
          <Button @click="incrementCount">Contador: {{ count }}</Button>
          <Button variant="secondary">Secundario</Button>
          <Button variant="outline">Outline</Button>
          <Button variant="ghost">Ghost</Button>
          <Button variant="destructive">Destructive</Button>
        </div>
      </div>

      <div class="space-y-4 p-6 border rounded-lg bg-card">
        <h2 class="text-xl font-semibold">TextArea</h2>
        <TextArea 
          v-model="message"
          placeholder="Escribe algo aquí para probar el textarea..." 
          class="min-h-[100px]"
        />
        <p class="text-sm text-muted-foreground" v-if="message">
          Has escrito: {{ message }}
        </p>
      </div>
    </div>
  </div>
</template>

