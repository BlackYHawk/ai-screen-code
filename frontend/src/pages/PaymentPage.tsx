import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { CheckCircle, XCircle, Loader2, RefreshCw } from 'lucide-react'
import { getOrderStatus, paymentCallback } from '@/api/client'
import type { CreateOrderResponse } from '@/types/api'

export function PaymentPage() {
  const { orderId } = useParams<{ orderId: string }>()
  const navigate = useNavigate()
  const [order, setOrder] = useState<CreateOrderResponse | null>(null)
  const [loading, setLoading] = useState(true)
  const [polling, setPolling] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (orderId) {
      loadOrder()
    }
  }, [orderId])

  useEffect(() => {
    if (order?.status === 'pending') {
      const interval = setInterval(() => {
        checkOrderStatus()
      }, 3000)
      return () => clearInterval(interval)
    }
  }, [order?.status])

  const loadOrder = async () => {
    try {
      const data = await getOrderStatus(orderId!)
      setOrder(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : '加载订单失败')
    } finally {
      setLoading(false)
    }
  }

  const checkOrderStatus = async () => {
    try {
      const data = await getOrderStatus(orderId!)
      setOrder(data)
    } catch (err) {
      console.error('Failed to check order status:', err)
    }
  }

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
      setError(err instanceof Error ? err.message : '支付失败')
    } finally {
      setPolling(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
      </div>
    )
  }

  if (error) {
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

  const paymentMethodNames: Record<string, string> = {
    alipay: '支付宝',
    wechat: '微信支付',
    yunshanfu: '云闪付',
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
                {order?.plan === 'lite' ? '基础版' : order?.plan === 'pro' ? '专业版' : '旗舰版'}
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

              {/* Simulate payment button for demo */}
              <button
                onClick={handleSimulatePayment}
                disabled={polling}
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
                disabled={polling}
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
