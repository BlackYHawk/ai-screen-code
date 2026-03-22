import { Minimize2, Stamp, UserCircle, Code2 } from 'lucide-react'
import type { Service } from '@/types/services'

export const services: Service[] = [
  {
    id: 'compress',
    title: '图片压缩',
    description: '智能压缩，支持 JPEG/PNG/WebP，保留高质量',
    icon: Minimize2,
    href: '/tools?tab=compress',
    type: 'consumer',
    ctaText: '立即使用',
  },
  {
    id: 'watermark',
    title: '图片水印',
    description: '批量添加文字或图片水印，防盗图',
    icon: Stamp,
    href: '/tools?tab=watermark',
    type: 'consumer',
    ctaText: '立即使用',
  },
  {
    id: 'portrait',
    title: '生成证件照',
    description: '一键生成标准证件照，支持多种尺寸',
    icon: UserCircle,
    href: '/tools?tab=portrait',
    type: 'consumer',
    ctaText: '立即使用',
  },
  {
    id: 'tocode',
    title: '图片转前端代码',
    description: 'AI 智能识别 UI 设计，生成 React + TypeScript 代码',
    icon: Code2,
    href: '/generate',
    type: 'developer',
    tags: ['React', 'TypeScript', 'Tailwind'],
    ctaText: '立即体验',
  },
]
