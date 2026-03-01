import { useState } from 'react'
import { Link } from 'react-router-dom'
import { useAppStore } from '@/stores/useAppStore'
import { Card, CardHeader, Button, Input } from '@/components/common'
import { validateApiKey } from '@/api/client'
import { toast } from 'sonner'
import type { AIModel } from '@/types/api'
import { Key, CheckCircle, XCircle, ArrowLeft } from 'lucide-react'

const models: AIModel[] = ['qwen', 'minimax', 'kimi', 'glm']

const modelLabels: Record<AIModel, string> = {
  qwen: 'Qwen (阿里云)',
  minimax: 'MiniMax',
  kimi: 'Kimi (月之暗面)',
  glm: 'GLM (智谱AI)',
}

export function SettingsPage() {
  const { config, setConfig } = useAppStore()
  const [validating, setValidating] = useState<AIModel | null>(null)
  const [validationResults, setValidationResults] = useState<Record<AIModel, boolean | null>>({
    qwen: null,
    minimax: null,
    kimi: null,
    glm: null,
  })

  const handleApiKeyChange = (model: AIModel, value: string) => {
    setConfig({
      api_keys: {
        ...config.api_keys,
        [model]: value,
      },
    })
    setValidationResults((prev) => ({ ...prev, [model]: null }))
  }

  const handleBaseUrlChange = (model: AIModel, value: string) => {
    setConfig({
      custom_base_urls: {
        ...config.custom_base_urls,
        [model]: value,
      },
    })
  }

  const handleValidate = async (model: AIModel) => {
    const apiKey = config.api_keys[model]
    if (!apiKey) {
      toast.error('请输入 API Key')
      return
    }

    setValidating(model)
    try {
      const result = await validateApiKey({
        model,
        api_key: apiKey,
        base_url: config.custom_base_urls[model] || undefined,
      })
      setValidationResults((prev) => ({ ...prev, [model]: result.valid }))
      if (result.valid) {
        toast.success('API Key 验证成功')
      } else {
        toast.error(result.message)
      }
    } catch (err) {
      setValidationResults((prev) => ({ ...prev, [model]: false }))
      const message = err instanceof Error ? err.message : '验证失败'
      toast.error(message)
    } finally {
      setValidating(null)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* 返回按钮 */}
        <div className="flex items-center mb-6">
          <Link
            to="/profile"
            className="p-2 hover:bg-gray-200 rounded-lg mr-2"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </Link>
          <h1 className="text-2xl font-bold text-gray-900">模型设置</h1>
        </div>

        <Card className="mb-6">
          <CardHeader>
            <div className="flex items-center space-x-2">
              <Key className="w-5 h-5" />
              <span>API Key 配置</span>
            </div>
          </CardHeader>
          <p className="text-sm text-gray-600 mb-6">
            配置各大AI模型的API Key，这些信息将保存在您的浏览器本地，不会上传到服务器。
          </p>

          <div className="space-y-6">
            {models.map((model) => (
              <div key={model} className="border-b border-gray-100 pb-6 last:border-0">
                <div className="flex items-center justify-between mb-3">
                  <label className="font-medium text-gray-900">
                    {modelLabels[model]}
                  </label>
                  {validationResults[model] === true && (
                    <span className="flex items-center text-sm text-green-600">
                      <CheckCircle className="w-4 h-4 mr-1" />
                      已验证
                    </span>
                  )}
                  {validationResults[model] === false && (
                    <span className="flex items-center text-sm text-red-600">
                      <XCircle className="w-4 h-4 mr-1" />
                      验证失败
                    </span>
                  )}
                </div>

                <div className="space-y-3">
                  <Input
                    type="password"
                    placeholder="输入 API Key"
                    value={config.api_keys[model]}
                    onChange={(e) => handleApiKeyChange(model, e.target.value)}
                  />
                  <div className="flex items-center space-x-3">
                    <Input
                      placeholder="自定义 Base URL (可选)"
                      value={config.custom_base_urls[model]}
                      onChange={(e) => handleBaseUrlChange(model, e.target.value)}
                      className="flex-1"
                    />
                    <Button
                      variant="outline"
                      onClick={() => handleValidate(model)}
                      isLoading={validating === model}
                      disabled={!config.api_keys[model]}
                    >
                      验证
                    </Button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>

        <Card>
          <CardHeader>使用说明</CardHeader>
          <ul className="text-sm text-gray-600 space-y-2">
            <li>1. 在各AI平台申请API Key并填写到对应配置中</li>
            <li>2. 点击"验证"按钮确认API Key是否有效</li>
            <li>3. 如果使用代理或自定义端点，可以填写自定义Base URL</li>
            <li>4. 您的API Key仅保存在浏览器本地，不会发送给任何服务器</li>
          </ul>
        </Card>
      </div>
    </div>
  )
}
