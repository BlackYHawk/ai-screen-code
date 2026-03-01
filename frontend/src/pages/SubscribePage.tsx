import { useState, useEffect, useCallback } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { Check, Zap, Crown, Rocket, Loader2, ArrowLeft } from 'lucide-react'
import { getPlans, createOrder, getSubscriptionStatus } from '@/api/client'
import type { SubscriptionPlan, SubscriptionStatusResponse, CreateOrderRequest } from '@/types/api'

type PlanId = 'lite' | 'pro' | 'max'

// Default plans when API fails
const defaultPlans: SubscriptionPlan[] = [
  { id: 'lite', name: '基础版', price: 9.9, price_display: '9.9', features: ['基础AI模型', '每日50次生成', '标准支持'] },
  { id: 'pro', name: '专业版', price: 29.9, price_display: '29.9', features: ['全部AI模型', '每日200次生成', '优先支持', '高级功能'] },
  { id: 'max', name: '旗舰版', price: 99, price_display: '99', features: ['全部AI模型', '无限次生成', '7x24小时支持', '企业级功能', '定制服务'] },
]

const planIcons: Record<PlanId, typeof Zap> = {
  lite: Zap,
  pro: Crown,
  max: Rocket,
}

// 固定颜色方案，避免 Tailwind 动态类名问题
const planColorClasses: Record<PlanId, {
  border: string
  bg: string
  button: string
  icon: string
  check: string
}> = {
  lite: {
    border: 'border-blue-500',
    bg: 'bg-blue-50',
    button: 'bg-blue-500 hover:bg-blue-600',
    icon: 'text-blue-500',
    check: 'text-blue-500',
  },
  pro: {
    border: 'border-purple-500',
    bg: 'bg-purple-50',
    button: 'bg-purple-500 hover:bg-purple-600',
    icon: 'text-purple-500',
    check: 'text-purple-500',
  },
  max: {
    border: 'border-orange-500',
    bg: 'bg-orange-50',
    button: 'bg-orange-500 hover:bg-orange-600',
    icon: 'text-orange-500',
    check: 'text-orange-500',
  },
}

const defaultColors = planColorClasses.lite

const planNameMap: Record<PlanId, string> = {
  lite: '基础版',
  pro: '专业版',
  max: '旗舰版',
}

export function SubscribePage() {
  const navigate = useNavigate()
  const [searchParams] = useSearchParams()
  const [plans, setPlans] = useState<SubscriptionPlan[]>([])
  const [subscriptionStatus, setSubscriptionStatus] = useState<SubscriptionStatusResponse | null>(null)
  const [selectedPlan, setSelectedPlan] = useState<PlanId | null>(null)
  const [selectedPaymentMethod, setSelectedPaymentMethod] = useState<string>('wechat')
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const loadData = useCallback(async () => {
    try {
      const [plansData, statusData] = await Promise.all([
        getPlans(),
        getSubscriptionStatus(),
      ])
      setPlans(plansData)
      setSubscriptionStatus(statusData)
    } catch {
      // Use default plans when API fails
      setPlans(defaultPlans)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    loadData()
  }, [loadData])

  // Initialize selected plan from URL params or default to cheapest
  useEffect(() => {
    if (plans.length > 0 && !selectedPlan) {
      const planParam = searchParams.get('plan')
      if (planParam && plans.find(p => p.id === planParam)) {
        setSelectedPlan(planParam as PlanId)
      } else {
        // Default to cheapest plan
        const cheapest = [...plans].sort((a, b) => a.price - b.price)[0]
        setSelectedPlan(cheapest.id as PlanId)
      }
    }
  }, [plans, selectedPlan, searchParams])

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
      const message = err instanceof Error ? err.message : '创建订单失败'
      setError(message)
    } finally {
      setSubmitting(false)
    }
  }

  const getSelectedPlanPrice = () => {
    if (!selectedPlan) return '0'
    const plan = plans.find(p => p.id === selectedPlan)
    return plan?.price_display || '0'
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
      {/* Back button */}
      <div className="flex items-center mb-6">
        <button
          onClick={() => navigate(-1)}
          className="p-2 hover:bg-gray-200 rounded-lg mr-2"
        >
          <ArrowLeft className="w-5 h-5 text-gray-600" />
        </button>
        <h1 className="text-2xl font-bold text-gray-900">订阅套餐</h1>
      </div>

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
      <div className="text-center mb-6">
        <p className="text-gray-600">
          解锁全部功能，让代码生成更高效
        </p>
      </div>

      {/* Error message */}
      {error && (
        <div className="mb-6 max-w-md mx-auto">
          <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-600 text-sm">{error}</p>
          </div>
        </div>
      )}

      {/* Plans */}
      <div className="grid md:grid-cols-3 gap-8 mb-12">
        {plans.map((plan) => {
          const planId = plan.id as PlanId
          const Icon = planIcons[planId] || Zap
          const colorClasses = planColorClasses[planId] || defaultColors
          const isSelected = selectedPlan === plan.id

          return (
            <div
              key={plan.id}
              className={`
                relative rounded-2xl border-2 p-6 cursor-pointer transition-all
                hover:shadow-lg
                ${isSelected
                  ? `${colorClasses.border} ${colorClasses.bg}`
                  : 'border-gray-200 bg-white hover:border-gray-300'
                }
              `}
              onClick={() => setSelectedPlan(planId)}
            >
              {plan.id === 'pro' && (
                <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                  <span className="px-3 py-1 bg-purple-500 text-white text-xs font-medium rounded-full">
                    最受欢迎
                  </span>
                </div>
              )}

              <div className="text-center mb-6">
                <Icon className={`w-12 h-12 mx-auto mb-4 ${colorClasses.icon}`} />
                <h3 className="text-xl font-bold text-gray-900 mb-2">
                  {planNameMap[planId] || plan.name}
                </h3>
                <div className="flex items-baseline justify-center">
                  <span className="text-4xl font-bold text-gray-900">¥{plan.price_display}</span>
                  <span className="text-gray-500 ml-1">/月</span>
                </div>
              </div>

              <ul className="space-y-3 mb-6">
                {plan.features.map((feature, index) => (
                  <li key={index} className="flex items-start">
                    <Check className={`w-5 h-5 ${colorClasses.check} mr-2 flex-shrink-0 mt-0.5`} />
                    <span className="text-gray-600">{feature}</span>
                  </li>
                ))}
              </ul>

              <button
                className={`
                  w-full py-3 rounded-lg font-medium transition-colors
                  ${isSelected
                    ? `${colorClasses.button} text-white`
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
          <h3 className="text-lg font-semibold text-gray-900 mb-3">选择支付方式</h3>

          <div className="flex space-x-3 mb-4">
            {[
              { id: 'wechat', label: '微信支付', icon: '💬' },
              { id: 'alipay', label: '支付宝', icon: '💳' },
              { id: 'yunshanfu', label: '云闪付', icon: '☁️' },
            ].map((method) => (
              <label
                key={method.id}
                className={`
                  flex-1 flex items-center justify-center p-3 border-2 rounded-lg cursor-pointer transition-colors
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
                <span className="text-xl mr-2">{method.icon}</span>
                <span className="font-medium text-gray-900 text-sm">{method.label}</span>
              </label>
            ))}
          </div>

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
              `立即支付 ¥${getSelectedPlanPrice()}`
            )}
          </button>
        </div>
      )}
    </div>
  )
}
