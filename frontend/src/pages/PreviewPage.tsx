import { useEffect, useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { Group as PanelGroup, Panel, Separator as PanelResizeHandle } from 'react-resizable-panels'
import { getHistoryById } from '@/api/client'
import { Layout } from '@/components/layout'
import { Card, CardHeader, PageLoader, Button } from '@/components/common'
import { CodeEditor, CodePreview } from '@/components/editor'
import { toast } from 'sonner'
import type { HistoryItem } from '@/types/api'
import { ArrowLeft } from 'lucide-react'

export function PreviewPage() {
  const { id } = useParams<{ id: string }>()
  const [history, setHistory] = useState<HistoryItem | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchHistory = async () => {
      if (!id) {
        setError('Invalid ID')
        setLoading(false)
        return
      }

      try {
        const data = await getHistoryById(id)
        setHistory(data)
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to load'
        setError(message)
        toast.error(message)
      } finally {
        setLoading(false)
      }
    }

    fetchHistory()
  }, [id])

  if (loading) {
    return (
      <Layout>
        <PageLoader />
      </Layout>
    )
  }

  if (error || !history) {
    return (
      <Layout>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <Link to="/history">
            <Button variant="ghost" className="mb-4">
              <ArrowLeft className="w-4 h-4 mr-2" />
              返回历史记录
            </Button>
          </Link>
          <Card>
            <p className="text-red-500">无法加载此记录</p>
          </Card>
        </div>
      </Layout>
    )
  }

  return (
    <Layout>
      <div className="h-[calc(100vh-64px)] flex flex-col">
        {/* Header */}
        <div className="px-4 sm:px-6 lg:px-8 py-4 border-b border-gray-200 bg-white">
          <div className="flex items-center justify-between max-w-7xl mx-auto">
            <div className="flex items-center">
              <Link to="/history">
                <Button variant="ghost" size="sm">
                  <ArrowLeft className="w-4 h-4 mr-2" />
                  返回
                </Button>
              </Link>
              <div className="ml-4">
                <h1 className="text-lg font-semibold text-gray-900">代码预览</h1>
                <p className="text-sm text-gray-500">
                  生成时间: {new Date(history.created_at).toLocaleString()}
                </p>
              </div>
            </div>
            <div className="flex items-center space-x-4 text-sm text-gray-600">
              <span>模型: {history.model.toUpperCase()}</span>
              <span>语言: {history.language.toUpperCase()}</span>
            </div>
          </div>
        </div>

        {/* Resizable Panels */}
        <div className="flex-1 p-4 overflow-hidden">
          <PanelGroup orientation="horizontal" className="h-full">
            {/* Code Editor Panel */}
            <Panel defaultSize={60} minSize={30}>
              <CodeEditor
                code={history.code}
                language={history.language}
                readOnly={true}
                showToolbar={true}
                height="100%"
              />
            </Panel>

            {/* Resize Handle */}
            <PanelResizeHandle className="w-1 bg-gray-200 hover:bg-blue-500 transition-colors cursor-col-resize" />

            {/* Preview Panel */}
            <Panel defaultSize={40} minSize={20}>
              <div className="h-full flex flex-col space-y-4">
                {/* Code Preview */}
                <div className="flex-1">
                  <CodePreview
                    code={history.code}
                    language={history.language}
                  />
                </div>

                {/* Original Image */}
                <div className="h-[200px]">
                  <Card className="h-full">
                    <CardHeader className="text-sm">原始图片</CardHeader>
                    <div className="flex-1 overflow-hidden rounded-lg">
                      <img
                        src={history.image}
                        alt="Original"
                        className="w-full h-full object-contain"
                      />
                    </div>
                  </Card>
                </div>
              </div>
            </Panel>
          </PanelGroup>
        </div>
      </div>
    </Layout>
  )
}
