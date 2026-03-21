import { useNavigate } from 'react-router-dom'
import { useState, useCallback, useEffect } from 'react'
import { useAppStore } from '@/stores/useAppStore'
import { generateCode } from '@/api/client'
import { Layout } from '@/components/layout'
import { Card, Button } from '@/components/common'
import { SubscriptionBanner } from '@/components/home'
import { CodeEditor } from '@/components/editor'
import { toast } from 'sonner'
import {
  Bot,
  Code2,
  Copy,
  Download,
  Check,
  Loader2,
  Sparkles,
  Zap,
  Eye,
  History,
} from 'lucide-react'

type GenerationStep = 'idle' | 'uploading' | 'analyzing' | 'generating' | 'complete' | 'error'

const stepLabels: Record<GenerationStep, string> = {
  idle: '等待开始',
  uploading: '上传图片',
  analyzing: '分析图片',
  generating: '生成代码',
  complete: '完成',
  error: '出错',
}

const modelNames: Record<string, string> = {
  qwen: 'Qwen (阿里云)',
  minimax: 'MiniMax',
  kimi: 'Kimi (月之暗面)',
  glm: 'GLM (智谱AI)',
}

const languageNames: Record<string, string> = {
  react: 'React + TypeScript',
  vue: 'Vue 3 + TypeScript',
  kotlin: 'Kotlin (Jetpack Compose)',
  swift: 'Swift (SwiftUI)',
}

export function GeneratePage() {
  const navigate = useNavigate()
  const {
    currentFile,
    selectedModel,
    selectedLanguage,
    config,
    isGenerating,
    setIsGenerating,
    setGenerateResult,
    setGenerateError,
  } = useAppStore()

  const [generationStep, setGenerationStep] = useState<GenerationStep>('idle')
  const [progress, setProgress] = useState(0)
  const [showResult, setShowResult] = useState(false)
  const [copied, setCopied] = useState(false)

  // Auto-start generation when page loads with a file
  useEffect(() => {
    if (currentFile && !isGenerating && !showResult) {
      handleGenerate()
    }
  }, [])

  const updateProgress = useCallback((step: GenerationStep, value: number) => {
    setGenerationStep(step)
    setProgress(value)
  }, [])

  const handleGenerate = async () => {
    if (!currentFile) {
      toast.error('请先上传图片')
      navigate('/')
      return
    }

    const apiKey = config.api_keys[selectedModel]

    // Reset state for new generation
    setIsGenerating(true)
    setShowResult(false)
    updateProgress('uploading', 10)

    try {
      updateProgress('analyzing', 30)

      // Use non-streaming API for code generation
      const response = await generateCode(
        {
          image: currentFile.base64,
          model: selectedModel,
          language: selectedLanguage,
          ...(apiKey ? { api_key: apiKey } : {}),
          custom_base_url: config.custom_base_urls[selectedModel] || undefined,
        }
      )

      updateProgress('complete', 100)

      // Store final result
      setGenerateResult({
        ...response,
        code: response.code,
      })
      setShowResult(true)
      toast.success('代码生成成功!')
    } catch (error) {
      const message = error instanceof Error ? error.message : '生成失败，请稍后重试'
      setGenerateError(message)
      updateProgress('error', 0)
      toast.error(message)
    } finally {
      setIsGenerating(false)
    }
  }

  const handleCopy = useCallback(async () => {
    const result = useAppStore.getState().generateResult
    if (result) {
      try {
        await navigator.clipboard.writeText(result.code)
        setCopied(true)
        setTimeout(() => setCopied(false), 2000)
      } catch (err) {
        console.error('Failed to copy:', err)
      }
    }
  }, [])

  const handleDownload = useCallback(() => {
    const result = useAppStore.getState().generateResult
    if (result) {
      const extensionMap: Record<string, string> = {
        kotlin: 'kt',
        react: 'tsx',
        swift: 'swift',
        vue: 'vue',
      }
      const extension = extensionMap[result.language]
      const filename = `generated-code.${extension}`
      const blob = new Blob([result.code], { type: 'text/plain' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = filename
      a.click()
      URL.revokeObjectURL(url)
    }
  }, [])

  const result = useAppStore.getState().generateResult

  // Determine which code to display
  const displayCode = result?.code || ''

  return (
    <Layout>
      <SubscriptionBanner />
      <div className="h-[calc(100vh-64px)] overflow-hidden">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-0 h-full">
          {/* Left Panel: Model Info & Progress */}
          <div className="lg:col-span-1 bg-gray-50 border-r border-gray-200 p-6 flex flex-col">
            <div className="mb-6">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">生成状态</h2>

              {/* Model Info Card */}
              <Card className="mb-4">
                <div className="flex items-center space-x-3">
                  <div className="p-2 bg-blue-100 rounded-lg">
                    <Bot className="w-5 h-5 text-blue-600" />
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">当前模型</p>
                    <p className="font-medium text-gray-900">{modelNames[selectedModel]}</p>
                  </div>
                </div>
                <div className="mt-3 flex items-center space-x-3">
                  <div className="p-2 bg-green-100 rounded-lg">
                    <Code2 className="w-5 h-5 text-green-600" />
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">目标语言</p>
                    <p className="font-medium text-gray-900">{languageNames[selectedLanguage]}</p>
                  </div>
                </div>
              </Card>
            </div>

            {/* Progress Section */}
            <div className="flex-1">
              <h3 className="text-sm font-medium text-gray-700 mb-3">生成进度</h3>

              {/* Animated Progress Display */}
              <Card className="bg-white">
                <div className="space-y-4">
                  {/* Status Icon */}
                  <div className="flex justify-center">
                    {isGenerating ? (
                      <div className="relative">
                        <div className="w-20 h-20 rounded-full border-4 border-blue-200" />
                        <div className="absolute inset-0 flex items-center justify-center">
                          <Loader2 className="w-10 h-10 text-blue-600 animate-spin" />
                        </div>
                      </div>
                    ) : generationStep === 'complete' ? (
                      <div className="w-20 h-20 rounded-full bg-green-100 flex items-center justify-center">
                        <Sparkles className="w-10 h-10 text-green-600" />
                      </div>
                    ) : generationStep === 'error' ? (
                      <div className="w-20 h-20 rounded-full bg-red-100 flex items-center justify-center">
                        <Zap className="w-10 h-10 text-red-600" />
                      </div>
                    ) : (
                      <div className="w-20 h-20 rounded-full bg-gray-100 flex items-center justify-center">
                        <Eye className="w-10 h-10 text-gray-400" />
                      </div>
                    )}
                  </div>

                  {/* Step Label */}
                  <div className="text-center">
                    <p className="text-lg font-medium text-gray-900">
                      {stepLabels[generationStep]}
                    </p>
                    {isGenerating && (
                      <p className="text-sm text-gray-500 mt-1">
                        请稍候...
                      </p>
                    )}
                  </div>

                  {/* Progress Bar */}
                  <div className="space-y-2">
                    <div className="h-2 bg-gray-100 rounded-full overflow-hidden">
                      <div
                        className={`h-full rounded-full transition-all duration-500 ease-out ${
                          generationStep === 'error'
                            ? 'bg-red-500'
                            : generationStep === 'complete'
                            ? 'bg-green-500'
                            : 'bg-blue-600'
                        }`}
                        style={{ width: `${progress}%` }}
                      />
                    </div>
                    <p className="text-right text-sm text-gray-500">{progress}%</p>
                  </div>

                  {/* Step Indicators */}
                  <div className="flex justify-between text-xs text-gray-400 pt-2">
                    <span className={generationStep !== 'idle' ? 'text-blue-600' : ''}>上传</span>
                    <span className={generationStep === 'analyzing' || generationStep === 'generating' || generationStep === 'complete' ? 'text-blue-600' : ''}>分析</span>
                    <span className={generationStep === 'generating' || generationStep === 'complete' ? 'text-blue-600' : ''}>生成</span>
                    <span className={generationStep === 'complete' ? 'text-green-600' : ''}>完成</span>
                  </div>
                </div>
              </Card>

              {/* Error Message */}
              {generationStep === 'error' && (
                <Card className="mt-4 bg-red-50 border-red-200">
                  <p className="text-sm text-red-600">
                    生成过程中出现错误，请检查 API Key 配置后重试
                  </p>
                  <Button
                    variant="outline"
                    size="sm"
                    className="mt-3 w-full"
                    onClick={() => navigate('/settings')}
                  >
                    前往设置
                  </Button>
                </Card>
              )}
            </div>

            {/* Action Buttons */}
            <div className="mt-auto pt-4 space-y-2">
              {showResult && result && (
                <>
                  <Button
                    variant="outline"
                    className="w-full"
                    onClick={handleCopy}
                  >
                    {copied ? (
                      <>
                        <Check className="w-4 h-4 mr-2" />
                        已复制
                      </>
                    ) : (
                      <>
                        <Copy className="w-4 h-4 mr-2" />
                        复制代码
                      </>
                    )}
                  </Button>
                  <Button
                    variant="outline"
                    className="w-full"
                    onClick={handleDownload}
                  >
                    <Download className="w-4 h-4 mr-2" />
                    下载代码
                  </Button>
                </>
              )}
              <Button
                variant="ghost"
                className="w-full"
                onClick={() => navigate('/')}
              >
                上传新图片
              </Button>
              <Button
                variant="ghost"
                className="w-full"
                onClick={() => navigate('/history')}
              >
                <History className="w-4 h-4 mr-2" />
                查看历史
              </Button>
            </div>
          </div>

          {/* Right Panel: Code Display */}
          <div className="lg:col-span-2 bg-white p-6 overflow-hidden">
            {/* Show result */}
            {showResult && result ? (
              <div className="h-full flex flex-col">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-lg font-semibold text-gray-900">
                    {isGenerating ? '正在生成...' : '生成的代码'}
                  </h2>
                  <span className="text-sm text-gray-500">
                    {languageNames[result?.language || selectedLanguage]}
                  </span>
                </div>
                <div className="flex-1 overflow-hidden">
                  <CodeEditor
                    code={displayCode}
                    language={result?.language || selectedLanguage}
                    readOnly={true}
                    showToolbar={true}
                    height="100%"
                  />
                </div>
              </div>
            ) : (
              <div className="h-full flex items-center justify-center">
                <div className="text-center text-gray-500">
                  {isGenerating ? (
                    <div className="space-y-4">
                      <div className="animate-spin w-16 h-16 border-4 border-blue-600 border-t-transparent rounded-full mx-auto" />
                      <p className="text-xl">正在 {stepLabels[generationStep]}...</p>
                      <p className="text-sm text-gray-400">
                        这可能需要几秒钟时间，请耐心等待
                      </p>
                    </div>
                  ) : (
                    <>
                      <Code2 className="w-16 h-16 mx-auto mb-4 text-gray-300" />
                      <p className="text-lg mb-2">等待生成</p>
                      <p className="text-sm">
                        {currentFile
                          ? '请点击左侧"生成代码"按钮开始'
                          : '请先上传图片'}
                      </p>
                    </>
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </Layout>
  )
}
