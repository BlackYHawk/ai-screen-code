import type {
  SubscriptionPlan,
  SubscriptionStatusResponse,
  CreateOrderRequest,
  CreateOrderResponse,
  PaymentCallbackRequest,
} from '@/types/api'

// Mock data
export const mockPlans: SubscriptionPlan[] = [
  {
    id: 'lite',
    name: 'lite',
    price: 1000,
    price_display: '10.00',
    features: ['基础代码生成', '每日50次生成', '标准支持'],
  },
  {
    id: 'pro',
    name: 'pro',
    price: 3000,
    price_display: '30.00',
    features: ['高级代码生成', '每日200次生成', '优先支持', '高级模板'],
  },
  {
    id: 'max',
    name: 'max',
    price: 5000,
    price_display: '50.00',
    features: ['全部功能', '无限次数生成', '7x24支持', '专属客服', 'API访问'],
  },
]

export const mockSubscriptionStatus: SubscriptionStatusResponse = {
  active: true,
  plan: 'pro',
  status: 'active',
  start_date: '2024-01-01 00:00:00',
  end_date: '2024-02-01 00:00:00',
}

export const mockInactiveSubscriptionStatus: SubscriptionStatusResponse = {
  active: false,
}

export const mockOrder: CreateOrderResponse = {
  order_id: 'test_order_123',
  plan: 'pro',
  amount: 3000,
  amount_display: '30.00',
  payment_method: 'alipay',
  status: 'pending',
  qr_code: 'alipay://mock_order_test_order_123',
  payment_url: '/payment/test_order_123',
}

export const mockPaidOrder: CreateOrderResponse = {
  order_id: 'test_order_123',
  plan: 'pro',
  amount: 3000,
  amount_display: '30.00',
  payment_method: 'alipay',
  status: 'paid',
}

// Mock functions
export const getPlans = async (): Promise<SubscriptionPlan[]> => {
  return Promise.resolve(mockPlans)
}

export const getSubscriptionStatus = async (): Promise<SubscriptionStatusResponse> => {
  return Promise.resolve(mockInactiveSubscriptionStatus)
}

export const createOrder = async (request: CreateOrderRequest): Promise<CreateOrderResponse> => {
  if (request.plan === 'invalid') {
    throw new Error('Invalid plan')
  }
  return Promise.resolve({
    ...mockOrder,
    plan: request.plan,
    payment_method: request.payment_method,
  })
}

export const getOrderStatus = async (orderId: string): Promise<CreateOrderResponse> => {
  if (orderId === 'invalid_order') {
    throw new Error('Order not found')
  }
  if (orderId === 'paid_order') {
    return Promise.resolve(mockPaidOrder)
  }
  return Promise.resolve(mockOrder)
}

export const paymentCallback = async (request: PaymentCallbackRequest): Promise<{ success: boolean }> => {
  if (request.order_id === 'invalid_order') {
    throw new Error('Order not found')
  }
  return Promise.resolve({ success: true })
}

export const getOrderHistory = async () => {
  return Promise.resolve([])
}

// Default export for compatibility
export default {
  getPlans,
  getSubscriptionStatus,
  createOrder,
  getOrderStatus,
  paymentCallback,
  getOrderHistory,
}
