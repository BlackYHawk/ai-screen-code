import type { LucideIcon } from 'lucide-react'

export interface Service {
  id: string
  title: string
  description: string
  icon: LucideIcon
  href: string
  type: 'consumer' | 'developer'
  tags?: string[]
  ctaText?: string
}

export interface ServiceCardProps {
  service: Service
  isDeveloperFeature?: boolean
}
