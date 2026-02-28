import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor, fireEvent } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { SubscribePage } from './SubscribePage'

// Mock the API client
const mockGetPlans = vi.fn()
const mockGetSubscriptionStatus = vi.fn()
const mockCreateOrder = vi.fn()

vi.mock('@/api/client', () => ({
  getPlans: (...args: unknown[]) => mockGetPlans(...args),
  getSubscriptionStatus: (...args: unknown[]) => mockGetSubscriptionStatus(...args),
  createOrder: (...args: unknown[]) => mockCreateOrder(...args),
}))

const defaultPlans = [
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

const defaultStatus = {
  active: false,
  plan: null,
  status: null,
  start_date: null,
  end_date: null,
}

const renderWithRouter = (component: React.ReactElement) => {
  return render(
    <MemoryRouter initialEntries={['/subscribe']}>
      <Routes>
        <Route path="/" element={<div>Home</div>} />
        <Route path="/subscribe" element={component} />
        <Route path="/payment/:orderId" element={<div>Payment Page</div>} />
      </Routes>
    </MemoryRouter>
  )
}

describe('SubscribePage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockGetPlans.mockResolvedValue(defaultPlans)
    mockGetSubscriptionStatus.mockResolvedValue(defaultStatus)
    mockCreateOrder.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock_order_test_order_123',
      payment_url: '/payment/test_order_123',
    })
  })

  it('renders subscription header', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('选择您的订阅套餐')).toBeInTheDocument()
    })
  })

  it('renders all three plans', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('基础版')).toBeInTheDocument()
      expect(screen.getByText('专业版')).toBeInTheDocument()
      expect(screen.getByText('旗舰版')).toBeInTheDocument()
    })
  })

  it('shows active subscription when user has one', async () => {
    mockGetSubscriptionStatus.mockResolvedValue({
      active: true,
      plan: 'pro',
      status: 'active',
      start_date: '2024-01-01',
      end_date: '2024-02-01',
    })

    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('当前订阅: pro')).toBeInTheDocument()
    })
  })

  it('allows selecting a plan', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('专业版')).toBeInTheDocument()
    })

    // Click on pro plan
    const proPlan = screen.getByText('专业版').closest('div[class*="cursor-pointer"]')
    if (proPlan) {
      fireEvent.click(proPlan)
    }

    // Check that "已选择" button appears
    await waitFor(() => {
      expect(screen.getByText('已选择')).toBeInTheDocument()
    })
  })

  it('shows payment method selection after selecting a plan', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('专业版')).toBeInTheDocument()
    })

    // Click on pro plan to select it
    const proPlan = screen.getByText('专业版').closest('div[class*="cursor-pointer"]')
    if (proPlan) {
      fireEvent.click(proPlan)
    }

    // Check that payment method selection appears
    await waitFor(() => {
      expect(screen.getByText('选择支付方式')).toBeInTheDocument()
      expect(screen.getByText('支付宝')).toBeInTheDocument()
      expect(screen.getByText('微信支付')).toBeInTheDocument()
      expect(screen.getByText('云闪付')).toBeInTheDocument()
    })
  })

  it('creates order when clicking subscribe button', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('专业版')).toBeInTheDocument()
    })

    // Click on pro plan to select it
    const proPlan = screen.getByText('专业版').closest('div[class*="cursor-pointer"]')
    if (proPlan) {
      fireEvent.click(proPlan)
    }

    // Click subscribe button
    await waitFor(() => {
      const button = screen.getByText(/立即支付/)
      fireEvent.click(button)
    })

    // Verify createOrder was called
    await waitFor(() => {
      expect(mockCreateOrder).toHaveBeenCalledWith({
        plan: 'pro',
        payment_method: 'alipay',
      })
    })
  })

  it('shows subscribe button after selecting a plan', async () => {
    renderWithRouter(<SubscribePage />)

    await waitFor(() => {
      expect(screen.getByText('选择您的订阅套餐')).toBeInTheDocument()
    })

    // Click on pro plan to select it
    const proPlan = screen.getByText('专业版').closest('div[class*="cursor-pointer"]')
    if (proPlan) {
      fireEvent.click(proPlan)
    }

    // Now the subscribe button should show the price
    await waitFor(() => {
      expect(screen.getByText(/立即支付 ¥30/)).toBeInTheDocument()
    })
  })

  it('handles API error gracefully', async () => {
    // Mock both calls to fail even after retry
    mockGetPlans
      .mockRejectedValueOnce(new Error('Network error'))
      .mockRejectedValueOnce(new Error('Network error'))

    renderWithRouter(<SubscribePage />)

    // Wait for loading to complete (with error)
    await waitFor(() => {
      expect(screen.queryByText('选择您的订阅套餐')).toBeInTheDocument()
    }, { timeout: 5000 })
  })
})
