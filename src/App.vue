<template>
  <GlobalMessage />
  <AnnouncementPopup />
  <router-view />
</template>

<script setup lang="ts">
import GlobalMessage from '@/components/GlobalMessage.vue'
import AnnouncementPopup from '@/components/AnnouncementPopup.vue'
</script>
