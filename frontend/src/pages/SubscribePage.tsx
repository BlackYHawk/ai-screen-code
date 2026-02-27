import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { Check, Zap, Crown, Rocket, Loader2 } from 'lucide-react'
import { getPlans, createOrder, getSubscriptionStatus } from '@/api/client'
import type { SubscriptionPlan, SubscriptionStatusResponse, CreateOrderRequest } from '@/types/api'

const planIcons = {
  lite: Zap,
  pro: Crown,
  max: Rocket,
}

const planColors = {
  lite: 'blue',
  pro: 'purple',
  max: 'orange',
}

export function SubscribePage() {
  const navigate = useNavigate()
  const [plans, setPlans] = useState<SubscriptionPlan[]>([])
  const [subscriptionStatus, setSubscriptionStatus] = useState<SubscriptionStatusResponse | null>(null)
  const [selectedPlan, setSelectedPlan] = useState<string | null>(null)
  const [selectedPaymentMethod, setSelectedPaymentMethod] = useState<string>('alipay')
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    try {
      const [plansData, statusData] = await Promise.all([
        getPlans(),
        getSubscriptionStatus(),
      ])
      setPlans(plansData)
      setSubscriptionStatus(statusData)
    } catch (err) {
      console.error('Failed to load data:', err)
    } finally {
      setLoading(false)
    }
  }

  const handleSubscribe = async () => {
    if (!selectedPlan) {
      setError('请选择订阅套餐')
      return
    }

    setSubmitting(true)
    setError(null)

    try {
      const request: CreateOrderRequest = {
        plan: selectedPlan,
        payment_method: selectedPaymentMethod,
      }
      const order = await createOrder(request)
      navigate(`/payment/${order.order_id}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : '创建订单失败')
    } finally {
      setSubmitting(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      {/* Current subscription status */}
      {subscriptionStatus?.active && (
        <div className="mb-8 p-4 bg-green-50 border border-green-200 rounded-lg">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="font-semibold text-green-800">当前订阅: {subscriptionStatus.plan}</h3>
              <p className="text-sm text-green-600">
                有效期至: {subscriptionStatus.end_date}
              </p>
            </div>
            <span className="px-3 py-1 bg-green-500 text-white text-sm rounded-full">
              {subscriptionStatus.status}
            </span>
          </div>
        </div>
      )}

      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">
          选择您的订阅套餐
        </h1>
        <p className="text-lg text-gray-600">
          解锁全部功能，让代码生成更高效
        </p>
      </div>

      {/* Plans */}
      <div className="grid md:grid-cols-3 gap-8 mb-12">
        {plans.map((plan) => {
          const Icon = planIcons[plan.id as keyof typeof planIcons] || Zap
          const color = planColors[plan.id as keyof typeof planColors] || 'blue'
          const isSelected = selectedPlan === plan.id

          return (
            <div
              key={plan.id}
              className={`
                relative rounded-2xl border-2 p-6 cursor-pointer transition-all
                hover:shadow-lg
                ${isSelected
                  ? `border-${color}-500 bg-${color}-50`
                  : 'border-gray-200 bg-white hover:border-gray-300'
                }
              `}
              onClick={() => setSelectedPlan(plan.id)}
            >
              {plan.id === 'pro' && (
                <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                  <span className="px-3 py-1 bg-purple-500 text-white text-xs font-medium rounded-full">
                    最受欢迎
                  </span>
                </div>
              )}

              <div className="text-center mb-6">
                <Icon className={`w-12 h-12 mx-auto mb-4 text-${color}-500`} />
                <h3 className="text-xl font-bold text-gray-900 mb-2">
                  {plan.name === 'lite' ? '基础版' : plan.name === 'pro' ? '专业版' : '旗舰版'}
                </h3>
                <div className="flex items-baseline justify-center">
                  <span className="text-4xl font-bold text-gray-900">¥{plan.price_display}</span>
                  <span className="text-gray-500 ml-1">/月</span>
                </div>
              </div>

              <ul className="space-y-3 mb-6">
                {plan.features.map((feature, index) => (
                  <li key={index} className="flex items-start">
                    <Check className={`w-5 h-5 text-${color}-500 mr-2 flex-shrink-0 mt-0.5`} />
                    <span className="text-gray-600">{feature}</span>
                  </li>
                ))}
              </ul>

              <button
                className={`
                  w-full py-3 rounded-lg font-medium transition-colors
                  ${isSelected
                    ? `bg-${color}-500 text-white`
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }
                `}
              >
                {isSelected ? '已选择' : '选择套餐'}
              </button>
            </div>
          )
        })}
      </div>

      {/* Payment method selection */}
      {selectedPlan && (
        <div className="max-w-md mx-auto">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">选择支付方式</h3>

          <div className="space-y-3 mb-6">
            {[
              { id: 'alipay', label: '支付宝', icon: '💳' },
              { id: 'wechat', label: '微信支付', icon: '💬' },
              { id: 'yunshanfu', label: '云闪付', icon: '☁️' },
            ].map((method) => (
              <label
                key={method.id}
                className={`
                  flex items-center p-4 border-2 rounded-lg cursor-pointer transition-colors
                  ${selectedPaymentMethod === method.id
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
                  }
                `}
              >
                <input
                  type="radio"
                  name="paymentMethod"
                  value={method.id}
                  checked={selectedPaymentMethod === method.id}
                  onChange={(e) => setSelectedPaymentMethod(e.target.value)}
                  className="sr-only"
                />
                <span className="text-2xl mr-3">{method.icon}</span>
                <span className="font-medium text-gray-900">{method.label}</span>
                {selectedPaymentMethod === method.id && (
                  <Check className="w-5 h-5 text-blue-500 ml-auto" />
                )}
              </label>
            ))}
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
              <p className="text-red-600 text-sm">{error}</p>
            </div>
          )}

          <button
            onClick={handleSubscribe}
            disabled={submitting}
            className="w-full py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
          >
            {submitting ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin mr-2" />
                正在创建订单...
              </>
            ) : (
              `立即支付 ¥${plans.find(p => p.id === selectedPlan)?.price_display || '0'}`
            )}
          </button>
        </div>
      )}
    </div>
  )
}
