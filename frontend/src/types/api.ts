// AI Model types
export type AIModel = 'qwen' | 'minimax' | 'kimi' | 'glm'

// Programming language types
export type ProgrammingLanguage = 'kotlin' | 'react' | 'swift' | 'vue'

// Generate request
export interface GenerateRequest {
  image: string // base64
  model: AIModel
  language: ProgrammingLanguage
  api_key?: string
  custom_base_url?: string
}

// Generate response
export interface GenerateResponse {
  id: string
  code: string
  language: ProgrammingLanguage
  preview_url?: string
}

// History item
export interface HistoryItem {
  id: string
  image: string // base64 or URL
  code: string
  language: ProgrammingLanguage
  model: AIModel
  created_at: string
}

// Model validation request
export interface ValidateModelRequest {
  model: AIModel
  api_key: string
  base_url?: string
}

// Model validation response
export interface ValidateModelResponse {
  valid: boolean
  message: string
}

// User config
export interface UserConfig {
  default_model: AIModel
  default_language: ProgrammingLanguage
  api_keys: Record<AIModel, string>
  custom_base_urls: Record<AIModel, string>
}

// API response wrapper
export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

// Uploaded file
export interface UploadedFile {
  id: string
  file: File
  preview: string
  base64: string
}

// Generate state
export interface GenerateState {
  isGenerating: boolean
  progress: number
  currentStep: string
  result?: GenerateResponse
  error?: string
}

// ============ Auth Types ============

// User
export interface User {
  id: string
  username: string
  email: string
  nickname?: string
  avatar?: string
  created_at: string
}

// Login request
export interface LoginRequest {
  email: string
  password: string
}

// Register request
export interface RegisterRequest {
  username: string
  email: string
  password: string
}

// Login response
export interface LoginResponse {
  token: string
  user: User
}

// Update profile request
export interface UpdateProfileRequest {
  nickname?: string
  avatar?: string
}

// Bank card
export interface BankCard {
  id: string
  card_number_last4: string
  bank_name: string
  card_holder_name: string
  created_at: string
}

// Bind card request
export interface BindCardRequest {
  card_number: string
  bank_name: string
  card_holder_name: string
}

// Subscription plan
export interface SubscriptionPlan {
  id: string
  name: string
  price: number
  price_display: string
  features: string[]
}

// Subscription status response
export interface SubscriptionStatusResponse {
  active: boolean
  plan?: string
  status?: string
  start_date?: string
  end_date?: string
}

// Create order request
export interface CreateOrderRequest {
  plan: string
  payment_method: string
}

// Create order response
export interface CreateOrderResponse {
  order_id: string
  amount: number
  amount_display: string
  payment_method: string
  qr_code?: string
  payment_url?: string
  status?: string
  plan?: string
}

// Order history item
export interface OrderHistoryItem {
  id: string
  plan: string
  amount: number
  amount_display: string
  payment_method: string
  status: string
  trade_no?: string
  created_at: string
}

// Payment callback request
export interface PaymentCallbackRequest {
  order_id: string
  trade_no: string
  status: string
}
