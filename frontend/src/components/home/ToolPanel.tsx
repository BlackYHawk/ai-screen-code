import { useState } from 'react'
import { Shield } from 'lucide-react'
import { CompressTool, WatermarkTool } from '@/components/tools'
import type { ToolType } from '@/types/image-tools'

const tools: { id: ToolType; label: string }[] = [
  { id: 'compress', label: '图片压缩' },
  { id: 'watermark', label: '图片水印' },
  // { id: 'portrait', label: '生成证件照' },
]

export function ToolPanel() {
  const [activeTab, setActiveTab] = useState<ToolType>('compress')

  return (
    <section className="py-12 bg-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-center gap-2 mb-6">
          <Shield className="w-5 h-5 text-green-600" />
          <span className="text-sm text-gray-600">图片本地处理，隐藏安全第一</span>
        </div>

        <div role="tablist" aria-label="图片处理工具" className="flex justify-center gap-2 mb-8">
          {tools.map((tool) => (
            <button
              key={tool.id}
              role="tab"
              aria-selected={activeTab === tool.id}
              aria-controls={`panel-${tool.id}`}
              onClick={() => setActiveTab(tool.id)}
              className={`px-6 py-3 rounded-lg font-medium transition-colors ${
                activeTab === tool.id
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {tool.label}
            </button>
          ))}
        </div>

        <div
          id={`panel-${activeTab}`}
          role="tabpanel"
          aria-labelledby={activeTab}
          className="min-h-[400px] border border-gray-200 rounded-xl p-6"
        >
          {activeTab === 'compress' && <CompressTool />}
          {activeTab === 'watermark' && <WatermarkTool />}
          {/* activeTab === 'portrait' && <PortraitTool /> */}
        </div>
      </div>
    </section>
  )
}
