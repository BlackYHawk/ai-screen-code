import { Shield, Zap, Lock } from 'lucide-react'

const trustFeatures = [
  {
    icon: Shield,
    title: '安全可靠',
    description: '端到端加密传输，保护您的数据安全',
  },
  {
    icon: Zap,
    title: '快速处理',
    description: '本地处理，无需上传到服务器',
  },
  {
    icon: Lock,
    title: '隐私保护',
    description: '图片仅在浏览器中处理，不会上传到任何服务器',
  },
]

export function TrustSection() {
  return (
    <section className="py-12 bg-white border-y border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-8">
          {trustFeatures.map((feature) => {
            const Icon = feature.icon
            return (
              <div key={feature.title} className="text-center">
                <div className="w-12 h-12 rounded-full bg-blue-50 text-blue-600 flex items-center justify-center mx-auto mb-4">
                  <Icon className="w-6 h-6" />
                </div>
                <h3 className="text-lg font-semibold text-gray-900">{feature.title}</h3>
                <p className="mt-2 text-gray-600 text-sm">{feature.description}</p>
              </div>
            )
          })}
        </div>
      </div>
    </section>
  )
}
