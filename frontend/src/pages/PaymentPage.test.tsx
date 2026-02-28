import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { PaymentPage } from './PaymentPage'

// Mock the API client
const mockGetOrderStatus = vi.fn()
const mockPaymentCallback = vi.fn()

vi.mock('@/api/client', () => ({
  getOrderStatus: (...args: unknown[]) => mockGetOrderStatus(...args),
  paymentCallback: (...args: unknown[]) => mockPaymentCallback(...args),
}))

const renderWithRouter = (orderId: string) => {
  return render(
    <MemoryRouter initialEntries={[`/payment/${orderId}`]}>
      <Routes>
        <Route path="/payment/:orderId" element={<PaymentPage />} />
        <Route path="/subscribe" element={<div>Subscribe Page</div>} />
        <Route path="/" element={<div>Home Page</div>} />
      </Routes>
    </MemoryRouter>
  )
}

describe('PaymentPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders order details after loading', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock',
      payment_url: '/payment/test_order_123',
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('等待支付')).toBeInTheDocument()
    })

    expect(screen.getByText('订单号')).toBeInTheDocument()
    expect(screen.getByText('test_order_123')).toBeInTheDocument()
  })

  it('shows payment success when order is paid', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'paid',
      qr_code: null,
      payment_url: null,
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('支付成功')).toBeInTheDocument()
      expect(screen.getByText('您的订阅已生效')).toBeInTheDocument()
    })
  })

  it('displays payment method name correctly', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock',
      payment_url: '/payment/test_order_123',
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('支付宝')).toBeInTheDocument()
    })
  })

  it('shows QR code for pending orders', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock_qr_code',
      payment_url: '/payment/test_order_123',
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('扫码支付')).toBeInTheDocument()
    })
  })

  it('shows simulate payment button for pending orders', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock',
      payment_url: '/payment/test_order_123',
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('模拟支付成功（演示）')).toBeInTheDocument()
    })
  })

  it('calls payment callback when simulate payment is clicked', async () => {
    mockGetOrderStatus
      .mockResolvedValueOnce({
        order_id: 'test_order_123',
        plan: 'pro',
        amount: 3000,
        amount_display: '30.00',
        payment_method: 'alipay',
        status: 'pending',
        qr_code: 'alipay://mock',
        payment_url: '/payment/test_order_123',
      })
      .mockResolvedValueOnce({
        order_id: 'test_order_123',
        plan: 'pro',
        amount: 3000,
        amount_display: '30.00',
        payment_method: 'alipay',
        status: 'paid',
        qr_code: null,
        payment_url: null,
      })

    mockPaymentCallback.mockResolvedValue({ success: true })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('模拟支付成功（演示）')).toBeInTheDocument()
    })

    const simulateButton = screen.getByText('模拟支付成功（演示）')
    fireEvent.click(simulateButton)

    await waitFor(() => {
      expect(mockPaymentCallback).toHaveBeenCalledWith({
        order_id: 'test_order_123',
        trade_no: expect.stringMatching(/^MOCK_\d+$/),
        status: 'paid',
      })
    })
  })

  it('shows refresh status button for pending orders', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'pending',
      qr_code: 'alipay://mock',
      payment_url: '/payment/test_order_123',
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('刷新状态')).toBeInTheDocument()
    })
  })

  it('shows return home button for paid orders', async () => {
    mockGetOrderStatus.mockResolvedValue({
      order_id: 'test_order_123',
      plan: 'pro',
      amount: 3000,
      amount_display: '30.00',
      payment_method: 'alipay',
      status: 'paid',
      qr_code: null,
      payment_url: null,
    })

    renderWithRouter('test_order_123')

    await waitFor(() => {
      expect(screen.getByText('返回首页')).toBeInTheDocument()
    })
  })
})
