import { createWebHashHistory, createWebHistory, type RouterHistory } from 'vue-router'

import { isTauriDesktop } from '@/utils/desktop-env'

type HistoryScope = Parameters<typeof isTauriDesktop>[0]

export function createAppHistory(scope?: HistoryScope): RouterHistory {
  if (isTauriDesktop(scope)) {
    return createWebHashHistory()
  }

  return createWebHistory()
}
