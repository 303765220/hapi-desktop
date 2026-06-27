// Hapi 对外展示“本站已稳定运行”的起算日，以北京时间 2026-04-28 作为业务口径。
// 改早会抬高展示天数，改晚会降低展示天数；如果真实上线口径变化，需要同步更新
// src/utils/__tests__/uptime.spec.ts 里的固定日期样本来验证展示值。
const HAPI_STABLE_SINCE = '2026-04-28T00:00:00+08:00'

// 按整天展示运行时长，避免小时/分钟造成侧边栏数字在一天内跳动。
// 改小会让展示频繁变化，改大则会低估运行天数；当前值由 uptime 单测覆盖。
const DAY_MS = 24 * 60 * 60 * 1000

export function getStableUptimeDays(now = new Date()): number {
  const start = new Date(HAPI_STABLE_SINCE)
  const diff = now.getTime() - start.getTime()
  return Math.max(0, Math.floor(diff / DAY_MS))
}
