import type { Group } from '@/types'

type UsageGroupLike = {
  group_id?: number | null
  group?: Pick<Group, 'name'> | null
}

export function formatUsageGroup(log: UsageGroupLike): string {
  const name = log.group?.name?.trim()
  if (name) {
    return name
  }
  if (log.group_id != null) {
    return `#${log.group_id}`
  }
  return '-'
}
