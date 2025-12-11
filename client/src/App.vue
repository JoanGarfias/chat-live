<script setup lang="ts">
import { computed, type Component } from 'vue'
import { RouterView, useRoute } from 'vue-router'

import DefaultLayout from '@/layouts/DefaultLayout.vue'
import AuthLayout from '@/layouts/AuthLayout.vue'

const route = useRoute()

const layouts: Record<string, Component> = {
  DefaultLayout,
  AuthLayout
}

const layout = computed(() => {
  const layoutName = (route.meta.layout as string) || 'DefaultLayout'
  console.log('Using layout:', layoutName)
  return layouts[layoutName] || DefaultLayout
})
</script>

<template>
  <component :is="layout">
    <router-view />
  </component>
</template>