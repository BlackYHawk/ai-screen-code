import axios, { type AxiosProgressEvent } from 'axios'
import type {
  GenerateRequest,
  GenerateResponse,
  HistoryItem,
  ValidateModelRequest,
  ValidateModelResponse,
  ApiResponse,
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  User,
  UpdateProfileRequest,
  BankCard,
  BindCardRequest,
  SubscriptionPlan,
  SubscriptionStatusResponse,
  CreateOrderRequest,
  CreateOrderResponse,
  OrderHistoryItem,
  PaymentCallbackRequest,
} from '@/types/api'

// Token management
let authToken: string | null = null

export const setAuthToken = (token: string | null) => {
  authToken = token
  if (token) {
    localStorage.setItem('auth_token', token)
  } else {
    localStorage.removeItem('auth_token')
  }
}

export const getAuthToken = (): string | null => {
  if (!authToken) {
    authToken = localStorage.getItem('auth_token')
  }
  return authToken
}

// Initialize token from storage
if (typeof window !== 'undefined') {
  getAuthToken()
}

const apiClient = axios.create({
  baseURL: '/api/v1',
  timeout: 300000, // 5 minutes for code generation
  headers: {
    'Content-Type': 'application/json',
  },
})

// Add auth token to requests
apiClient.interceptors.request.use((config) => {
  const token = getAuthToken()
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Generate code from image
export const generateCode = async (request: GenerateRequest): Promise<GenerateResponse> => {
  const response = await apiClient.post<ApiResponse<GenerateResponse>>('/generate', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to generate code')
  }
  return response.data.data!
}

// Generate code with streaming
export const generateCodeStream = async (
  request: GenerateRequest,
  onChunk: (chunk: string) => void,
  onProgress?: (progress: number) => void
): Promise<GenerateResponse> => {
  const response = await apiClient.post<ApiResponse<GenerateResponse>>(
    '/generate',
    { ...request, stream: true },
    {
      responseType: 'stream',
      onDownloadProgress: (progressEvent: AxiosProgressEvent) => {
        if (onProgress && progressEvent.total) {
          const progress = Math.round((progressEvent.loaded / progressEvent.total) * 100)
          onProgress(progress)
        }
      },
    }
  )

  return new Promise((resolve, reject) => {
    let fullCode = ''
    const stream = response.data as unknown as {
      on: (event: string, callback: (chunk: Buffer) => void) => void
    }

    stream.on('data', (chunk: Buffer) => {
      const text = chunk.toString()
      fullCode += text
      onChunk(text)
    })

    stream.on('end', () => {
      try {
        const data = JSON.parse(fullCode)
        if (data.success) {
          resolve(data.data)
        } else {
          reject(new Error(data.error || 'Failed to generate code'))
        }
      } catch {
        reject(new Error('Invalid response format'))
      }
    })

    stream.on('error', (err: unknown) => {
      reject(err instanceof Error ? err : new Error('Stream error'))
    })
  })
}

// Validate API key
export const validateApiKey = async (
  request: ValidateModelRequest
): Promise<ValidateModelResponse> => {
  const response = await apiClient.post<ApiResponse<ValidateModelResponse>>(
    '/models/validate',
    request
  )
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to validate API key')
  }
  return response.data.data!
}

// Get history
export const getHistory = async (): Promise<HistoryItem[]> => {
  const response = await apiClient.get<ApiResponse<HistoryItem[]>>('/history')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get history')
  }
  return response.data.data || []
}

// Delete history item
export const deleteHistory = async (id: string): Promise<{ success: boolean }> => {
  const response = await apiClient.delete<ApiResponse<{ success: boolean }>>(`/history/${id}`)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to delete history')
  }
  return response.data.data!
}

// Get history by ID
export const getHistoryById = async (id: string): Promise<HistoryItem> => {
  const response = await apiClient.get<ApiResponse<HistoryItem>>(`/history/${id}`)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get history item')
  }
  return response.data.data!
}

// ============ Auth API ============

// Register
export const register = async (request: RegisterRequest): Promise<LoginResponse> => {
  const response = await apiClient.post<ApiResponse<LoginResponse>>('/auth/register', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Registration failed')
  }
  const data = response.data.data!
  setAuthToken(data.token)
  return data
}

// Login
export const login = async (request: LoginRequest): Promise<LoginResponse> => {
  const response = await apiClient.post<ApiResponse<LoginResponse>>('/auth/login', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Login failed')
  }
  const data = response.data.data!
  setAuthToken(data.token)
  return data
}

// Logout
export const logout = () => {
  setAuthToken(null)
}

// Get current user profile
export const getProfile = async (): Promise<User> => {
  const response = await apiClient.get<ApiResponse<User>>('/auth/profile')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get profile')
  }
  return response.data.data!
}

// Update profile
export const updateProfile = async (request: UpdateProfileRequest): Promise<User> => {
  const response = await apiClient.put<ApiResponse<User>>('/auth/profile', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to update profile')
  }
  return response.data.data!
}

// List bank cards
export const listCards = async (): Promise<BankCard[]> => {
  const response = await apiClient.get<ApiResponse<BankCard[]>>('/auth/cards')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get cards')
  }
  return response.data.data || []
}

// Bind bank card
export const bindCard = async (request: BindCardRequest): Promise<BankCard> => {
  const response = await apiClient.post<ApiResponse<BankCard>>('/auth/cards', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to bind card')
  }
  return response.data.data!
}

// Delete bank card
export const deleteCard = async (cardId: string): Promise<{ success: boolean }> => {
  const response = await apiClient.delete<ApiResponse<{ success: boolean }>>(`/auth/cards/${cardId}`)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to delete card')
  }
  return response.data.data!
}

// ============ Subscription API ============

// Get subscription plans
export const getPlans = async (): Promise<SubscriptionPlan[]> => {
  const response = await apiClient.get<ApiResponse<SubscriptionPlan[]>>('/subscriptions/plans')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get plans')
  }
  return response.data.data || []
}

// Get current subscription status
export const getSubscriptionStatus = async (): Promise<SubscriptionStatusResponse> => {
  const response = await apiClient.get<ApiResponse<SubscriptionStatusResponse>>('/subscriptions/status')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get subscription status')
  }
  return response.data.data!
}

// Create order
export const createOrder = async (request: CreateOrderRequest): Promise<CreateOrderResponse> => {
  const response = await apiClient.post<ApiResponse<CreateOrderResponse>>('/subscriptions/create', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to create order')
  }
  return response.data.data!
}

// Get order status
export const getOrderStatus = async (orderId: string): Promise<CreateOrderResponse> => {
  const response = await apiClient.get<ApiResponse<CreateOrderResponse>>(`/subscriptions/orders/${orderId}`)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get order status')
  }
  return response.data.data!
}

// Get order history
export const getOrderHistory = async (): Promise<OrderHistoryItem[]> => {
  const response = await apiClient.get<ApiResponse<OrderHistoryItem[]>>('/subscriptions/orders')
  if (!response.data.success) {
    throw new Error(response.data.error || 'Failed to get order history')
  }
  return response.data.data || []
}

// Payment callback (simulate payment)
export const paymentCallback = async (request: PaymentCallbackRequest): Promise<{ success: boolean }> => {
  const response = await apiClient.post<ApiResponse<{ success: boolean }>>('/subscriptions/webhook', request)
  if (!response.data.success) {
    throw new Error(response.data.error || 'Payment callback failed')
  }
  return response.data.data!
}

export default apiClient
