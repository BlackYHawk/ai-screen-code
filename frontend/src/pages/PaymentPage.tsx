import { useState, useEffect, useCallback, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { CheckCircle, XCircle, Loader2, RefreshCw, AlertTriangle } from 'lucide-react'
import { getOrderStatus, paymentCallback } from '@/api/client'
import type { CreateOrderResponse } from '@/types/api'

const MAX_POLL_COUNT = 60 // 最多轮询 3 分钟 (60 * 3s)
const POLL_INTERVAL = 3000 // 3 秒轮询一次

const paymentMethodNames: Record<string, string> = {
  alipay: '支付宝',
  wechat: '微信支付',
  yunshanfu: '云闪付',
}

const planNameMap: Record<string, string> = {
  lite: '基础版',
  pro: '专业版',
  max: '旗舰版',
}

export function PaymentPage() {
  const { orderId } = useParams<{ orderId: string }>()
  const navigate = useNavigate()
  const [order, setOrder] = useState<CreateOrderResponse | null>(null)
  const [loading, setLoading] = useState(true)
  const [polling, setPolling] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [pollCount, setPollCount] = useState(0)
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const loadOrder = useCallback(async () => {
    if (!orderId) return

    try {
      const data = await getOrderStatus(orderId)
      setOrder(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : '加载订单失败'
      setError(message)
      // 重试一次
      try {
        const data = await getOrderStatus(orderId)
        setOrder(data)
        setError(null)
      } catch {
        // 保留原始错误
      }
    } finally {
      setLoading(false)
    }
  }, [orderId])

  const checkOrderStatus = useCallback(async () => {
    if (!orderId) return

    try {
      const data = await getOrderStatus(orderId)
      setOrder(data)

      // 如果订单已支付，停止轮询
      if (data.status === 'paid') {
        setPolling(false)
        if (intervalRef.current) {
          clearInterval(intervalRef.current)
          intervalRef.current = null
        }
      }
    } catch (err) {
      // 静默处理轮询错误，避免频繁提示
      console.error('Failed to check order status:', err)
      // 轮询失败时不设置error，避免干扰用户
    }
  }, [orderId])

  // 初始加载
  useEffect(() => {
    if (orderId) {
      loadOrder()
    }
  }, [orderId, loadOrder])

  // 轮询订单状态
  useEffect(() => {
    if (order?.status === 'pending' && pollCount < MAX_POLL_COUNT && !polling) {
      intervalRef.current = setInterval(() => {
        setPollCount((prev) => {
          if (prev >= MAX_POLL_COUNT) {
            if (intervalRef.current) {
              clearInterval(intervalRef.current)
            }
            return prev
          }
          checkOrderStatus()
          return prev + 1
        })
      }, POLL_INTERVAL)

      return () => {
        if (intervalRef.current) {
          clearInterval(intervalRef.current)
        }
      }
    }
  }, [order?.status, pollCount, polling, checkOrderStatus])

  // 清理函数
  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [])

  const handleSimulatePayment = async () => {
    if (!orderId) return

    setPolling(true)
    try {
      await paymentCallback({
        order_id: orderId,
        trade_no: `MOCK_${Date.now()}`,
        status: 'paid',
      })
      await checkOrderStatus()
    } catch (err) {
      const message = err instanceof Error ? err.message : '支付失败'
      setError(message)
    } finally {
      setPolling(false)
    }
  }

  const isPollingTimeout = pollCount >= MAX_POLL_COUNT

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
      </div>
    )
  }

  if (error && !order) {
    return (
      <div className="max-w-md mx-auto px-4 py-12">
        <div className="text-center">
          <XCircle className="w-16 h-16 text-red-500 mx-auto mb-4" />
          <h2 className="text-xl font-bold text-gray-900 mb-2">加载失败</h2>
          <p className="text-gray-600 mb-6">{error}</p>
          <button
            onClick={() => navigate('/subscribe')}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            返回订阅页面
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-lg mx-auto px-4 py-8">
      <div className="bg-white rounded-2xl border border-gray-200 p-6">
        {/* Status */}
        <div className="text-center mb-8">
          {order?.status === 'paid' ? (
            <>
              <CheckCircle className="w-16 h-16 text-green-500 mx-auto mb-4" />
              <h2 className="text-2xl font-bold text-gray-900 mb-2">支付成功</h2>
              <p className="text-gray-600">您的订阅已生效</p>
            </>
          ) : order?.status === 'pending' ? (
            <>
              <Loader2 className="w-16 h-16 text-blue-500 mx-auto mb-4 animate-spin" />
              <h2 className="text-2xl font-bold text-gray-900 mb-2">等待支付</h2>
              <p className="text-gray-600">请使用 {paymentMethodNames[order?.payment_method || '']} 完成支付</p>
            </>
          ) : (
            <>
              <XCircle className="w-16 h-16 text-red-500 mx-auto mb-4" />
              <h2 className="text-2xl font-bold text-gray-900 mb-2">支付失败</h2>
              <p className="text-gray-600">订单状态: {order?.status}</p>
            </>
          )}
        </div>

        {/* Polling timeout warning */}
        {isPollingTimeout && order?.status === 'pending' && (
          <div className="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg flex items-center gap-2">
            <AlertTriangle className="w-5 h-5 text-yellow-600 flex-shrink-0" />
            <p className="text-sm text-yellow-700">支付超时，请重新下单</p>
          </div>
        )}

        {/* Error message */}
        {error && (
          <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-600 text-sm">{error}</p>
          </div>
        )}

        {/* Order details */}
        <div className="border-t border-gray-200 pt-6 mb-6">
          <h3 className="font-semibold text-gray-900 mb-4">订单详情</h3>
          <dl className="space-y-3">
            <div className="flex justify-between">
              <dt className="text-gray-600">订单号</dt>
              <dd className="font-medium text-gray-900">{order?.order_id}</dd>
            </div>
            <div className="flex justify-between">
              <dt className="text-gray-600">套餐</dt>
              <dd className="font-medium text-gray-900">
                {planNameMap[order?.plan || ''] || order?.plan}
              </dd>
            </div>
            <div className="flex justify-between">
              <dt className="text-gray-600">支付方式</dt>
              <dd className="font-medium text-gray-900">
                {paymentMethodNames[order?.payment_method || '']}
              </dd>
            </div>
            <div className="flex justify-between">
              <dt className="text-gray-600">金额</dt>
              <dd className="font-bold text-xl text-blue-600">¥{order?.amount_display}</dd>
            </div>
          </dl>
        </div>

        {/* Payment QR Code (simulated) */}
        {order?.status === 'pending' && order?.qr_code && (
          <div className="border-t border-gray-200 pt-6 mb-6">
            <h3 className="font-semibold text-gray-900 mb-4 text-center">扫码支付</h3>
            <div className="bg-gray-50 rounded-lg p-4 text-center">
              <div className="w-48 h-48 mx-auto bg-white border-2 border-gray-200 rounded-lg flex items-center justify-center mb-4">
                <span className="text-4xl">📱</span>
              </div>
              <p className="text-sm text-gray-600 mb-4">
                使用{paymentMethodNames[order.payment_method]}扫描上方二维码完成支付
              </p>

              {/* Simulate payment button - only in dev mode */}
              {import.meta.env.DEV && (
                <button
                  onClick={handleSimulatePayment}
                  disabled={polling || isPollingTimeout}
                  className="px-6 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 disabled:opacity-50 flex items-center justify-center mx-auto"
                >
                  {polling ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin mr-2" />
                      模拟支付中...
                    </>
                  ) : (
                    <>
                      <RefreshCw className="w-4 h-4 mr-2" />
                      模拟支付成功（演示）
                    </>
                  )}
                </button>
              )}
            </div>
          </div>
        )}

        {/* Actions */}
        <div className="flex gap-4">
          {order?.status === 'paid' ? (
            <button
              onClick={() => navigate('/')}
              className="flex-1 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700"
            >
              返回首页
            </button>
          ) : (
            <>
              <button
                onClick={() => navigate('/subscribe')}
                className="flex-1 py-3 bg-gray-100 text-gray-700 rounded-lg font-medium hover:bg-gray-200"
              >
                重新选择
              </button>
              <button
                onClick={checkOrderStatus}
                disabled={polling || isPollingTimeout}
                className="flex-1 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 disabled:opacity-50 flex items-center justify-center"
              >
                <RefreshCw className={`w-4 h-4 mr-2 ${polling ? 'animate-spin' : ''}`} />
                刷新状态
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  )
}
