import { useNavigate } from 'react-router-dom'
import { useAppStore } from '@/stores/useAppStore'
import { Layout } from '@/components/layout'
import { Card, Button, Select } from '@/components/common'
import { ImageUploadWithPreview } from '@/components/upload'
import { Sparkles } from 'lucide-react'
import type { AIModel, ProgrammingLanguage } from '@/types/api'

const modelOptions = [
  { value: 'qwen', label: 'Qwen (阿里云)' },
  { value: 'minimax', label: 'MiniMax' },
  { value: 'kimi', label: 'Kimi (月之暗面)' },
  { value: 'glm', label: 'GLM (智谱AI)' },
]

const languageOptions = [
  { value: 'react', label: 'React + TypeScript' },
  { value: 'vue', label: 'Vue 3 + TypeScript' },
  { value: 'kotlin', label: 'Kotlin (Jetpack Compose)' },
  { value: 'swift', label: 'Swift (SwiftUI)' },
]

export function HomePage() {
  const navigate = useNavigate()
  const { currentFile, setCurrentFile, selectedModel, setSelectedModel, selectedLanguage, setSelectedLanguage } = useAppStore()

  const handleFileUpload = (file: File) => {
    const reader = new FileReader()
    reader.onload = () => {
      setCurrentFile({
        id: crypto.randomUUID(),
        file,
        preview: reader.result as string,
        base64: (reader.result as string).split(',')[1],
      })
    }
    reader.readAsDataURL(file)
  }

  const handleRemoveFile = () => {
    setCurrentFile(null)
  }

  const handleStartGenerate = () => {
    navigate('/generate')
  }

  return (
    <Layout>
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">
            AI Screen Code
          </h1>
          <p className="text-lg text-gray-600">
            上传UI设计图片，AI自动生成前端代码
          </p>
        </div>

        <Card padding="lg">
          {/* Model and Language Selection */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
            <Select
              label="选择 AI 模型"
              value={selectedModel}
              onChange={(e) => setSelectedModel(e.target.value as AIModel)}
              options={modelOptions}
            />
            <Select
              label="目标语言"
              value={selectedLanguage}
              onChange={(e) => setSelectedLanguage(e.target.value as ProgrammingLanguage)}
              options={languageOptions}
            />
          </div>

          <ImageUploadWithPreview
            onUpload={handleFileUpload}
            currentFile={currentFile}
            onRemove={handleRemoveFile}
          />

          <div className="mt-6 flex justify-center">
            <Button
              onClick={handleStartGenerate}
              disabled={!currentFile}
              size="lg"
              className="flex items-center space-x-2"
            >
              <Sparkles className="w-5 h-5" />
              <span>生成代码</span>
            </Button>
          </div>

          {!currentFile && (
            <p className="text-center text-sm text-gray-500 mt-4">
              请先上传一张UI设计图片
            </p>
          )}
        </Card>

        <div className="mt-8 grid grid-cols-2 gap-4">
          <Card className="text-center">
            <h3 className="font-semibold text-gray-900 mb-2">多种模型</h3>
            <p className="text-sm text-gray-600">
              支持 Qwen、MiniMax、Kimi、GLM 等AI模型
            </p>
          </Card>
          <Card className="text-center">
            <h3 className="font-semibold text-gray-900 mb-2">多语言输出</h3>
            <p className="text-sm text-gray-600">
              React、Vue、Swift、Kotlin 多种框架
            </p>
          </Card>
        </div>
      </div>
    </Layout>
  )
}
