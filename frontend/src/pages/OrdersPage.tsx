import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { List, Loader2, CheckCircle, Clock, XCircle, ArrowLeft, Sparkles } from 'lucide-react'
import { getSubscriptionStatus, getOrderHistory, getPlans } from '@/api/client'
import type { SubscriptionStatusResponse, OrderHistoryItem, SubscriptionPlan } from '@/types/api'

const PLAN_NAMES: Record<string, string> = {
  lite: '基础版',
  pro: '专业版',
  max: '旗舰版',
}

const PLAN_PRICES: Record<string, string> = {
  lite: '9.9',
  pro: '29.9',
  max: '99',
}

const PAYMENT_METHOD_NAMES: Record<string, string> = {
  alipay: '支付宝',
  wechat: '微信支付',
  yunshanfu: '云闪付',
}

const STATUS_CONFIG: Record<string, { icon: typeof CheckCircle; color: string; label: string }> = {
  paid: { icon: CheckCircle, color: 'green', label: '已支付' },
  pending: { icon: Clock, color: 'yellow', label: '待支付' },
  cancelled: { icon: XCircle, color: 'red', label: '已取消' },
}

const PLAN_DESCRIPTIONS: Record<string, string> = {
  lite: '适合轻度使用，包含基础功能',
  pro: '适合日常使用，解锁全部功能',
  max: '适合重度使用，享受最高权益',
}

export function OrdersPage() {
  const navigate = useNavigate()
  const [subscription, setSubscription] = useState<SubscriptionStatusResponse | null>(null)
  const [orders, setOrders] = useState<OrderHistoryItem[]>([])
  const [plans, setPlans] = useState<SubscriptionPlan[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false

    const loadData = async () => {
      try {
        const [subData, orderData] = await Promise.all([
          getSubscriptionStatus(),
          getOrderHistory()
        ])
        if (!cancelled) {
          setSubscription(subData)
          setOrders(orderData)
        }
      } catch (err) {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : '加载数据失败')
        }
      }

      // Load plans separately (don't fail if this errors)
      try {
        const plansData = await getPlans()
        if (!cancelled) {
          setPlans(plansData)
        }
      } catch {
        // Silently fail for plans - we have fallback data
      }

      if (!cancelled) {
        setLoading(false)
      }
    }

    loadData()

    return () => {
      cancelled = true
    }
  }, [])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header with back button */}
      <div className="flex items-center mb-8">
        <button
          onClick={() => navigate('/profile')}
          className="flex items-center justify-center w-10 h-10 rounded-lg bg-gray-100 hover:bg-gray-200 transition-colors mr-4"
        >
          <ArrowLeft className="w-5 h-5 text-gray-600" />
        </button>
        <h1 className="text-2xl font-bold text-gray-900">我的订阅</h1>
      </div>

      {/* Current Subscription */}
      <div className="mb-8">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">当前套餐</h2>
        {subscription?.active ? (
          <div className="bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl p-6 text-white">
            <div className="flex items-start justify-between">
              <div>
                <div className="flex items-center gap-2 mb-2">
                  <Sparkles className="w-5 h-5" />
                  <span className="text-lg font-semibold">
                    {PLAN_NAMES[subscription.plan || ''] || subscription.plan}
                  </span>
                </div>
                <p className="text-blue-100 text-sm mb-4">
                  {PLAN_DESCRIPTIONS[subscription.plan || '']}
                </p>
                <div className="text-sm text-blue-100">
                  <p>有效期至：{subscription.end_date}</p>
                </div>
              </div>
              <div className="px-3 py-1 bg-white/20 rounded-full text-sm">
                生效中
              </div>
            </div>
          </div>
        ) : (
          <div className="bg-gray-50 border border-gray-200 rounded-xl p-6">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium text-gray-900 mb-1">暂无有效订阅</h3>
                <p className="text-gray-500 text-sm">订阅后享受更多功能和服务</p>
              </div>
              <button
                onClick={() => navigate('/subscribe')}
                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
              >
                立即订阅
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Plans List - Show when no active subscription */}
      {(!subscription || !subscription.active) && !loading && (
        <div className="mb-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">选择套餐</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {(plans.length > 0 ? plans : [
              { id: 'lite', name: '基础版', price: 9.9, price_display: '9.9', features: ['基础功能'] },
              { id: 'pro', name: '专业版', price: 29.9, price_display: '29.9', features: ['全部功能'] },
              { id: 'max', name: '旗舰版', price: 99, price_display: '99', features: ['高级功能'] }
            ] as SubscriptionPlan[]).map((plan) => (
              <div
                key={plan.id}
                className="bg-white border border-gray-200 rounded-xl p-4 hover:shadow-lg transition-shadow cursor-pointer"
                onClick={() => navigate(`/subscribe?plan=${plan.id}`)}
              >
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-semibold text-gray-900">{PLAN_NAMES[plan.id] || plan.id}</h3>
                  <span className="text-xl font-bold text-blue-600">¥{PLAN_PRICES[plan.id] || plan.price}</span>
                </div>
                <p className="text-sm text-gray-500 mb-4">{PLAN_DESCRIPTIONS[plan.id]}</p>
                <button className="w-full py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-sm">
                  立即订阅
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Subscription History */}
      <div>
        <h2 className="text-lg font-semibold text-gray-900 mb-4">订阅记录</h2>

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
          <p className="text-gray-500">您还没有任何订阅订单</p>
        </div>
      ) : (
        <div className="space-y-4">
          {orders.map((order) => {
            const status = STATUS_CONFIG[order.status] || STATUS_CONFIG.pending
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
                        {PLAN_NAMES[order.plan] || order.plan}
                      </h3>
                      <span className={`px-2 py-0.5 rounded-full text-xs flex items-center ${
                        status.color === 'green' ? 'bg-green-100 text-green-700' :
                        status.color === 'yellow' ? 'bg-yellow-100 text-yellow-700' :
                        'bg-red-100 text-red-700'
                      }`}>
                        <StatusIcon className={`w-3 h-3 mr-1`} />
                        {status.label}
                      </span>
                    </div>
                    <div className="flex items-center gap-6 text-sm text-gray-500">
                      <span>订单号: {order.id.slice(0, 8)}...</span>
                      <span>支付方式: {PAYMENT_METHOD_NAMES[order.payment_method]}</span>
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
    </div>
  )
}
