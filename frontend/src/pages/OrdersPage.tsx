import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { List, Loader2, CheckCircle, Clock, XCircle } from 'lucide-react'
import { getOrderHistory } from '@/api/client'
import type { OrderHistoryItem } from '@/types/api'

export function OrdersPage() {
  const navigate = useNavigate()
  const [orders, setOrders] = useState<OrderHistoryItem[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadOrders()
  }, [])

  const loadOrders = async () => {
    try {
      const data = await getOrderHistory()
      setOrders(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : '加载订单失败')
    } finally {
      setLoading(false)
    }
  }

  const planNames: Record<string, string> = {
    lite: '基础版',
    pro: '专业版',
    max: '旗舰版',
  }

  const paymentMethodNames: Record<string, string> = {
    alipay: '支付宝',
    wechat: '微信支付',
    yunshanfu: '云闪付',
  }

  const statusConfig: Record<string, { icon: typeof CheckCircle; color: string; label: string }> = {
    paid: { icon: CheckCircle, color: 'green', label: '已支付' },
    pending: { icon: Clock, color: 'yellow', label: '待支付' },
    cancelled: { icon: XCircle, color: 'red', label: '已取消' },
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div className="flex items-center">
          <List className="w-6 h-6 text-gray-600 mr-2" />
          <h1 className="text-2xl font-bold text-gray-900">订单历史</h1>
        </div>
        <button
          onClick={() => navigate('/subscribe')}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          立即订阅
        </button>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
          <p className="text-red-600">{error}</p>
        </div>
      )}

      {orders.length === 0 ? (
        <div className="text-center py-12">
          <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <List className="w-8 h-8 text-gray-400" />
          </div>
          <h3 className="text-lg font-medium text-gray-900 mb-2">暂无订单</h3>
          <p className="text-gray-500 mb-6">您还没有任何订阅订单</p>
          <button
            onClick={() => navigate('/subscribe')}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            立即订阅
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {orders.map((order) => {
            const status = statusConfig[order.status] || statusConfig.pending
            const StatusIcon = status.icon

            return (
              <div
                key={order.id}
                className="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
              >
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-3 mb-2">
                      <h3 className="font-semibold text-gray-900">
                        {planNames[order.plan] || order.plan}
                      </h3>
                      <span className={`px-2 py-0.5 bg-${status.color}-100 text-${status.color}-700 text-xs rounded-full flex items-center`}>
                        <StatusIcon className={`w-3 h-3 mr-1`} />
                        {status.label}
                      </span>
                    </div>
                    <div className="flex items-center gap-6 text-sm text-gray-500">
                      <span>订单号: {order.id.slice(0, 8)}...</span>
                      <span>支付方式: {paymentMethodNames[order.payment_method]}</span>
                      <span>创建时间: {order.created_at}</span>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-xl font-bold text-blue-600">¥{order.amount_display}</div>
                    {order.trade_no && (
                      <div className="text-xs text-gray-500 mt-1">
                        交易号: {order.trade_no.slice(0, 12)}...
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )
          })}
        </div>
      )}
    </div>
  )
}
