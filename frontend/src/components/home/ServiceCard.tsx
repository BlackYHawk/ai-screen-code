import { Link } from 'react-router-dom'
import { ArrowRight } from 'lucide-react'
import type { Service } from '@/types/services'

interface ServiceCardProps {
  service: Service
  isDeveloperFeature?: boolean
}

export function ServiceCard({ service, isDeveloperFeature = false }: ServiceCardProps) {
  const Icon = service.icon

  if (isDeveloperFeature || service.type === 'developer') {
    return (
      <div className="group relative bg-gradient-to-br from-gray-900 to-gray-800 rounded-2xl p-6 text-white shadow-lg hover:shadow-xl hover:-translate-y-1 transition-all duration-200">
        <div className="absolute inset-0 bg-gradient-to-br from-blue-600/10 to-purple-600/10 rounded-2xl" />
        <div className="relative">
          <div className="w-12 h-12 rounded-xl bg-blue-500/20 text-blue-400 flex items-center justify-center mb-4">
            <Icon className="w-6 h-6" />
          </div>
          <h3 className="text-lg font-semibold">{service.title}</h3>
          <p className="mt-2 text-gray-400 text-sm">{service.description}</p>
          {service.tags && service.tags.length > 0 && (
            <div className="mt-4 flex flex-wrap gap-2">
              {service.tags.map((tag) => (
                <span
                  key={tag}
                  className="px-2 py-1 bg-blue-500/20 text-blue-400 text-xs rounded font-mono"
                >
                  {tag}
                </span>
              ))}
            </div>
          )}
          <div className="mt-6 flex items-center justify-between">
            <Link
              to={service.href}
              className="inline-flex items-center text-blue-400 font-medium hover:text-blue-300 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-400 focus-visible:ring-offset-2 focus-visible:ring-offset-gray-900 rounded"
            >
              {service.ctaText || '立即使用'}
              <ArrowRight className="ml-1 w-4 h-4" />
            </Link>
            {service.id === 'tocode' && (
              <div className="flex gap-2 text-xs text-gray-500">
                <Link to="/docs" className="hover:text-gray-300">文档</Link>
                <Link to="/demo" className="hover:text-gray-300">演示</Link>
              </div>
            )}
          </div>
        </div>
      </div>
    )
  }

  return (
    <Link
      to={service.href}
      className="group relative bg-white rounded-2xl p-6 shadow-sm border border-gray-200 hover:shadow-lg hover:border-blue-200 hover:-translate-y-1 transition-all duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
    >
      <div className="w-12 h-12 rounded-xl bg-blue-50 text-blue-600 flex items-center justify-center mb-4">
        <Icon className="w-6 h-6" />
      </div>
      <h3 className="text-lg font-semibold text-gray-900">{service.title}</h3>
      <p className="mt-2 text-gray-600 text-sm">{service.description}</p>
      <div className="mt-4 flex items-center text-blue-600 text-sm font-medium">
        {service.ctaText || '立即使用'}
        <ArrowRight className="ml-1 w-4 h-4 group-hover:translate-x-1 transition-transform" />
      </div>
    </Link>
  )
}
