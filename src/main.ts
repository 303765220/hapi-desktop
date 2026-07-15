import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import './style.css'
import App from './App.vue'
import { useAuthStore } from '@/stores/auth'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
useAuthStore().checkAuth()
app.use(router)

app.mount('#app')

// Prevent mouse wheel / trackpad pinch zoom
document.addEventListener(
  'wheel',
  function (e) {
    if (e.ctrlKey || e.metaKey) {
      e.preventDefault()
    }
  },
  { passive: false }
)

// Prevent Safari gesture zoom
document.addEventListener('gesturestart', function (e) {
  e.preventDefault()
})

// Prevent keyboard zoom shortcuts
document.addEventListener('keydown', function (e) {
  if (
    (e.ctrlKey || e.metaKey) &&
    (e.key === '=' || e.key === '-' || e.key === '0')
  ) {
    e.preventDefault()
  }
})
