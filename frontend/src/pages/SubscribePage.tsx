import { useState, useEffect, useCallback } from 'react'
import { useNavigate, useSearchParams, Link } from 'react-router-dom'
import { Check, Zap, Crown, Rocket, Loader2, Sparkles, ArrowLeft } from 'lucide-react'
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
  bgLight: string
  button: string
  icon: string
  check: string
  gradient: string
}> = {
  lite: {
    border: 'border-blue-500',
    bg: 'bg-blue-500',
    bgLight: 'bg-blue-50',
    button: 'bg-blue-500 hover:bg-blue-600',
    icon: 'text-blue-500',
    check: 'text-blue-500',
    gradient: 'from-blue-500/10 to-transparent',
  },
  pro: {
    border: 'border-purple-500',
    bg: 'bg-purple-500',
    bgLight: 'bg-purple-50',
    button: 'bg-purple-500 hover:bg-purple-600',
    icon: 'text-purple-500',
    check: 'text-purple-500',
    gradient: 'from-purple-500/10 to-transparent',
  },
  max: {
    border: 'border-orange-500',
    bg: 'bg-orange-500',
    bgLight: 'bg-orange-50',
    button: 'bg-orange-500 hover:bg-orange-600',
    icon: 'text-orange-500',
    check: 'text-orange-500',
    gradient: 'from-orange-500/10 to-transparent',
  },
}

const defaultColors = planColorClasses.lite

const planNameMap: Record<PlanId, string> = {
  lite: '基础版',
  pro: '专业版',
  max: '旗舰版',
}

const paymentMethods = [
  { id: 'wechat', label: '微信支付', icon: '💬' },
  { id: 'alipay', label: '支付宝', icon: '💳' },
  { id: 'yunshanfu', label: '云闪付', icon: '☁️' },
]

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
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-white to-purple-50">
        <div className="text-center">
          <Loader2 className="w-10 h-10 animate-spin text-blue-600 mx-auto mb-3" />
          <p className="text-gray-500">加载中...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50 py-6">
      <div className="max-w-5xl mx-auto px-4">
        {/* Header with back button */}
        <div className="flex items-center mb-6">
          <button
            onClick={() => navigate(-1)}
            className="flex items-center justify-center w-10 h-10 rounded-xl bg-white shadow-sm hover:shadow-md transition-shadow mr-4"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <div>
            <h1 className="text-2xl font-bold text-gray-900">订阅套餐</h1>
          </div>
        </div>

        {/* Current subscription status */}
        {subscriptionStatus?.active && (
          <div className="mb-6 p-4 bg-white/80 backdrop-blur-sm border border-green-200 rounded-xl shadow-sm">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-green-100 rounded-full flex items-center justify-center">
                  <Check className="w-5 h-5 text-green-600" />
                </div>
                <div>
                  <h3 className="font-semibold text-gray-900">当前订阅: {subscriptionStatus.plan}</h3>
                  <p className="text-sm text-gray-500">有效期至: {subscriptionStatus.end_date}</p>
                </div>
              </div>
              <span className="px-3 py-1.5 bg-green-500 text-white text-sm font-medium rounded-full">
                {subscriptionStatus.status}
              </span>
            </div>
          </div>
        )}

        {/* Plans header */}
        <div className="text-center mb-6">
          <p className="text-gray-500">解锁全部AI能力，让代码生成更高效</p>
        </div>

        {/* Plans */}
        <div className="grid md:grid-cols-3 gap-4 mb-6">
          {plans.map((plan) => {
            const planId = plan.id as PlanId
            const Icon = planIcons[planId] || Zap
            const colorClasses = planColorClasses[planId] || defaultColors
            const isSelected = selectedPlan === plan.id
            const isPro = plan.id === 'pro'

            return (
              <div
                key={plan.id}
                className={`
                  relative rounded-2xl border-2 p-5 cursor-pointer transition-all duration-300
                  hover:shadow-xl hover:-translate-y-1
                  ${isSelected
                    ? `${colorClasses.border} ${colorClasses.bgLight} shadow-lg`
                    : 'border-gray-100 bg-white/80 backdrop-blur hover:border-gray-300'
                  }
                `}
                onClick={() => setSelectedPlan(planId)}
              >
                {/* Popular badge */}
                {isPro && (
                  <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                    <span className="px-4 py-1 bg-gradient-to-r from-purple-500 to-purple-600 text-white text-xs font-medium rounded-full shadow-lg">
                      最受欢迎
                    </span>
                  </div>
                )}

                {/* Selected indicator */}
                {isSelected && (
                  <div className="absolute top-3 right-3">
                    <div className={`w-5 h-5 ${colorClasses.bg} rounded-full flex items-center justify-center`}>
                      <Check className="w-3 h-3 text-white" />
                    </div>
                  </div>
                )}

                <div className="text-center mb-4">
                  <div className={`
                    inline-flex items-center justify-center w-12 h-12 rounded-xl mb-3
                    ${isSelected ? `${colorClasses.bg}` : 'bg-gray-100'}
                    transition-colors duration-300
                  `}>
                    <Icon className={`w-6 h-6 ${isSelected ? 'text-white' : colorClasses.icon}`} />
                  </div>
                  <h3 className="text-lg font-bold text-gray-900 mb-1">
                    {planNameMap[planId] || plan.name}
                  </h3>
                  <div className="flex items-baseline justify-center">
                    <span className="text-3xl font-bold text-gray-900">¥{plan.price_display}</span>
                    <span className="text-gray-400 text-sm ml-1">/月</span>
                  </div>
                </div>

                <ul className="space-y-2 mb-4">
                  {plan.features.map((feature, index) => (
                    <li key={index} className="flex items-center">
                      <Check className={`w-4 h-4 ${colorClasses.check} mr-2 flex-shrink-0`} />
                      <span className="text-gray-600 text-sm">{feature}</span>
                    </li>
                  ))}
                </ul>

                <button
                  className={`
                    w-full py-2.5 rounded-xl font-medium transition-all duration-300
                    ${isSelected
                      ? `${colorClasses.button} text-white shadow-lg shadow-${planId === 'lite' ? 'blue' : planId === 'pro' ? 'purple' : 'orange'}-500/25`
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                    }
                  `}
                >
                  {isSelected ? '✓ 已选择' : '选择套餐'}
                </button>
              </div>
            )
          })}
        </div>

        {/* Payment & Action */}
        {selectedPlan && (
          <div className="max-w-2xl mx-auto">
            {/* Error message - above pay button */}
            {error && (
              <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-xl">
                <p className="text-red-600 text-sm">{error}</p>
              </div>
            )}

            {/* Selected plan summary */}
            <div className="bg-white/80 backdrop-blur-sm border border-gray-200 rounded-xl p-5 mb-4">
              <div className="flex items-center justify-between mb-4">
                <span className="text-gray-500">已选套餐</span>
                <span className="font-semibold text-gray-900 text-lg">
                  {planNameMap[selectedPlan as PlanId]} - ¥{getSelectedPlanPrice()}/月
                </span>
              </div>

              {/* Payment methods */}
              <div className="space-y-3">
                <p className="text-sm text-gray-500 font-medium">支付方式</p>
                <div className="grid grid-cols-3 gap-3">
                  {paymentMethods.map((method) => (
                    <label
                      key={method.id}
                      className={`
                        flex items-center justify-center p-3 border-2 rounded-xl cursor-pointer transition-all duration-200
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
                      <span className="font-medium text-gray-900">{method.label}</span>
                    </label>
                  ))}
                </div>
              </div>
            </div>

            {/* Submit button */}
            <button
              onClick={handleSubscribe}
              disabled={submitting}
              className="w-full py-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-xl font-semibold hover:from-blue-600 hover:to-purple-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center shadow-lg shadow-blue-500/25 transition-all duration-300 hover:shadow-xl"
            >
              {submitting ? (
                <>
                  <Loader2 className="w-5 h-5 animate-spin mr-2" />
                  正在创建订单...
                </>
              ) : (
                <>
                  <Sparkles className="w-5 h-5 mr-2" />
                  立即支付 ¥{getSelectedPlanPrice()}
                </>
              )}
            </button>

            <p className="text-center text-sm text-gray-400 mt-4">
              支付即表示您同意我们的
              <Link to="/terms" className="text-blue-500 hover:underline mx-1">服务条款</Link>
              和
              <Link to="/privacy" className="text-blue-500 hover:underline mx-1">隐私政策</Link>
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
