import { createRouter } from 'vue-router'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

NProgress.configure({ showSpinner: false, speed: 400, minimum: 0.1 })
import UserLayout from '@/layout/UserLayout.vue'
import { createAppHistory } from './history'
import { resolveAuthRedirect } from './auth-guard'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createAppHistory(),
  routes: [
    {
      path: '/login',
      name: 'Login',
      component: () => import('@/views/auth/LoginView.vue')
    },
    {
      path: '/',
      component: UserLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'Dashboard',
          component: () => import('@/views/DashboardView.vue'),
        },
        {
          path: 'subscriptions',
          name: '我的订阅',
          component: () => import('@/views/SubscriptionsView.vue'),
        },
        {
          path: 'keys',
          name: 'API Keys',
          component: () => import('@/views/KeysView.vue'),
        },
        {
          path: 'usage',
          name: 'Usage',
          component: () => import('@/views/UsageView.vue'),
        },
        {
          path: 'billing',
          name: 'Billing',
          component: () => import('@/views/BillingView.vue'),
        },
        {
          path: 'profile',
          name: 'Profile',
          component: () => import('@/views/ProfileView.vue'),
        },
        {
          path: 'models',
          name: 'Models',
          component: () => import('@/views/ModelsView.vue'),
        },
        {
          path: 'affiliate',
          name: 'Affiliate',
          component: () => import('@/views/AffiliateView.vue'),
        },
        {
          path: 'client-setup',
          name: '一键配置',
          component: () => import('@/views/ClientSetupView.vue'),
        },
        {
          path: 'status',
          name: 'ChannelStatus',
          component: () => import('@/views/ChannelStatusView.vue'),
        },
        {
          path: 'subscriptions',
          name: 'Subscriptions',
          component: () => import('@/views/SubscriptionsView.vue'),
        }
      ]
    }
  ]
})

router.beforeEach((to, _from, next) => {
  NProgress.start()
  const authStore = useAuthStore()
  const redirect = resolveAuthRedirect(to, {
    isAuthenticated: authStore.isAuthenticated,
    isAdmin: authStore.isAdmin,
  })
  if (redirect) {
    next(redirect)
    return
  }
  next()
})

router.afterEach(() => {
  NProgress.done()
})

export default router
