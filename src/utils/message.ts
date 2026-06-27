import { ref } from 'vue'

export type MessageType = 'success' | 'error' | 'info' | 'warning'

export interface MessageOptions {
  id: number
  type: MessageType
  text: string
  duration?: number
}

export const messages = ref<MessageOptions[]>([])

let id = 0

export const showMessage = (text: string, type: MessageType = 'info', duration = 3000) => {
  const msgId = ++id
  messages.value.push({ id: msgId, type, text, duration })
  if (duration > 0) {
    setTimeout(() => {
      removeMessage(msgId)
    }, duration)
  }
}

export const removeMessage = (msgId: number) => {
  messages.value = messages.value.filter(m => m.id !== msgId)
}

export const useMessage = () => {
  return {
    success: (text: string, duration?: number) => showMessage(text, 'success', duration),
    error: (text: string, duration?: number) => showMessage(text, 'error', duration),
    info: (text: string, duration?: number) => showMessage(text, 'info', duration),
    warning: (text: string, duration?: number) => showMessage(text, 'warning', duration)
  }
}

// Confirm Dialog
export interface ConfirmOptions {
  title: string
  content: string
  confirmText?: string
  cancelText?: string
  onConfirm: () => void
  onCancel?: () => void
}

export const confirmState = ref<ConfirmOptions | null>(null)

export const showConfirm = (options: ConfirmOptions) => {
  confirmState.value = options
}

export const closeConfirm = () => {
  confirmState.value = null
}
