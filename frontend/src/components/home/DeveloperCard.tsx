import { Link } from 'react-router-dom'
import { Code2 } from 'lucide-react'

export function DeveloperCard() {
  return (
    <section className="py-12 bg-gradient-to-br from-gray-900 to-gray-800">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="bg-gradient-to-br from-gray-900 to-gray-800 rounded-2xl p-8 border border-gray-700">
          <div className="flex flex-col md:flex-row items-center gap-6">
            <div className="flex-shrink-0">
              <div className="w-16 h-16 bg-blue-600 rounded-xl flex items-center justify-center">
                <Code2 className="w-8 h-8 text-white" />
              </div>
            </div>

            <div className="flex-1 text-center md:text-left">
              <h2 className="text-2xl font-bold text-white mb-2">图片转前端代码</h2>
              <p className="text-gray-400 mb-4">
                AI驱动的图片到前端代码转换服务，快速将设计稿转换为可用的React组件
              </p>

              <div className="flex flex-wrap gap-2 justify-center md:justify-start mb-6">
                <span className="px-3 py-1 bg-gray-700 text-gray-300 text-sm rounded-full font-mono">
                  React
                </span>
                <span className="px-3 py-1 bg-gray-700 text-gray-300 text-sm rounded-full font-mono">
                  TypeScript
                </span>
                <span className="px-3 py-1 bg-gray-700 text-gray-300 text-sm rounded-full font-mono">
                  Tailwind
                </span>
              </div>
            </div>

            <div className="flex-shrink-0">
              <Link
                to="/generate"
                className="inline-flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
              >
                开始使用
                <Code2 className="w-5 h-5" />
              </Link>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
