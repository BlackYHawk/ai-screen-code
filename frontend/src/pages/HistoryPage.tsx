import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { getHistory, deleteHistory } from '@/api/client'
import { Layout } from '@/components/layout'
import { Card, Button, PageLoader } from '@/components/common'
import { toast } from 'sonner'
import type { HistoryItem } from '@/types/api'
import { Trash2, Eye, Code } from 'lucide-react'

export function HistoryPage() {
  const [history, setHistory] = useState<HistoryItem[]>([])
  const [loading, setLoading] = useState(true)
  const [deletingId, setDeletingId] = useState<string | null>(null)

  const fetchHistory = async () => {
    try {
      const data = await getHistory()
      setHistory(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to load history'
      toast.error(message)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchHistory()
  }, [])

  const handleDelete = async (id: string) => {
    setDeletingId(id)
    try {
      await deleteHistory(id)
      setHistory((prev) => prev.filter((item) => item.id !== id))
      toast.success('删除成功')
    } catch (err) {
      const message = err instanceof Error ? err.message : '删除失败'
      toast.error(message)
    } finally {
      setDeletingId(null)
    }
  }

  if (loading) {
    return (
      <Layout>
        <PageLoader />
      </Layout>
    )
  }

  return (
    <Layout>
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <h1 className="text-2xl font-bold text-gray-900 mb-6">历史记录</h1>

        {history.length === 0 ? (
          <Card>
            <div className="text-center py-12">
              <Code className="w-12 h-12 text-gray-300 mx-auto mb-4" />
              <p className="text-gray-500">暂无历史记录</p>
              <Link to="/">
                <Button className="mt-4">开始生成</Button>
              </Link>
            </div>
          </Card>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {history.map((item) => (
              <Card key={item.id} padding="none">
                <div className="aspect-video relative overflow-hidden rounded-t-xl bg-gray-100">
                  <img
                    src={item.image}
                    alt="Preview"
                    className="w-full h-full object-cover"
                  />
                </div>
                <div className="p-4">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-xs font-medium px-2 py-1 bg-blue-100 text-blue-700 rounded">
                      {item.model.toUpperCase()}
                    </span>
                    <span className="text-xs font-medium px-2 py-1 bg-green-100 text-green-700 rounded">
                      {item.language.toUpperCase()}
                    </span>
                  </div>
                  <p className="text-xs text-gray-500 mb-3">
                    {new Date(item.created_at).toLocaleString()}
                  </p>
                  <div className="flex items-center justify-between">
                    <Link to={`/preview/${item.id}`}>
                      <Button variant="outline" size="sm">
                        <Eye className="w-4 h-4 mr-1" />
                        查看
                      </Button>
                    </Link>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleDelete(item.id)}
                      disabled={deletingId === item.id}
                    >
                      <Trash2 className="w-4 h-4 text-red-500" />
                    </Button>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>
    </Layout>
  )
}
