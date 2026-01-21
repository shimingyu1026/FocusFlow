import type { FocusSession } from '@/types/database'

export interface DailyStats {
  date: string
  total_minutes: number
  count: number
}

export interface TagStats {
  tag: string
  total_minutes: number
  percentage: number
}

export function formatMinutes(minutes: number): string {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours}小时${mins}分钟`
  }
  return `${mins}分钟`
}

export function calculateTagStats(sessions: FocusSession[]): TagStats[] {
  const tagTotals: Record<string, number> = {}
  let totalAll = 0

  sessions.forEach(session => {
    session.tags.forEach(tag => {
      tagTotals[tag] = (tagTotals[tag] || 0) + session.duration
      totalAll += session.duration
    })
  })

  return Object.entries(tagTotals).map(([tag, total_minutes]) => ({
    tag,
    total_minutes,
    percentage: totalAll > 0 ? (total_minutes / totalAll) * 100 : 0
  }))
}
