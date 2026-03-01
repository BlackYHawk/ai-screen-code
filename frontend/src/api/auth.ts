import {
  ApiResponse,
  BankCard,
  BindCardRequest,
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  UpdateProfileRequest,
  User,
} from '@/types/api'

const authClient = {
  // Get token from localStorage
  getToken(): string | null {
    return localStorage.getItem('auth_token')
  },

  // Set token to localStorage
  setToken(token: string): void {
    localStorage.setItem('auth_token', token)
  },

  // Remove token from localStorage
  removeToken(): void {
    localStorage.removeItem('auth_token')
  },

  // Get stored user
  getStoredUser(): User | null {
    const userStr = localStorage.getItem('auth_user')
    if (userStr) {
      try {
        return JSON.parse(userStr)
      } catch {
        return null
      }
    }
    return null
  },

  // Store user
  setStoredUser(user: User): void {
    localStorage.setItem('auth_user', JSON.stringify(user))
  },

  // Remove stored user
  removeStoredUser(): void {
    localStorage.removeItem('auth_user')
  },

  // Check if user is logged in
  isLoggedIn(): boolean {
    return !!this.getToken()
  },

  // Register
  async register(data: RegisterRequest): Promise<LoginResponse> {
    const response = await fetch('/api/v1/auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    })

    const result: ApiResponse<LoginResponse> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Registration failed')
    }

    // Store token and user
    if (result.data) {
      this.setToken(result.data.token)
      this.setStoredUser(result.data.user)
    }

    return result.data!
  },

  // Login
  async login(data: LoginRequest): Promise<LoginResponse> {
    const response = await fetch('/api/v1/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    })

    const result: ApiResponse<LoginResponse> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Login failed')
    }

    // Store token and user
    if (result.data) {
      this.setToken(result.data.token)
      this.setStoredUser(result.data.user)
    }

    return result.data!
  },

  // Logout
  logout(): void {
    this.removeToken()
    this.removeStoredUser()
  },

  // Get auth headers
  getAuthHeaders(): HeadersInit {
    const token = this.getToken()
    return {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    }
  },

  // Get profile
  async getProfile(): Promise<User> {
    const response = await fetch('/api/v1/auth/profile', {
      method: 'GET',
      headers: this.getAuthHeaders(),
    })

    const result: ApiResponse<User> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to get profile')
    }

    if (result.data) {
      this.setStoredUser(result.data)
    }

    return result.data!
  },

  // Update profile
  async updateProfile(data: UpdateProfileRequest): Promise<User> {
    const response = await fetch('/api/v1/auth/profile', {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(data),
    })

    const result: ApiResponse<User> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to update profile')
    }

    if (result.data) {
      this.setStoredUser(result.data)
    }

    return result.data!
  },

  // Upload avatar
  async uploadAvatar(file: File): Promise<{ url: string }> {
    const formData = new FormData()
    formData.append('avatar', file)

    const token = this.getToken()
    const response = await fetch('/api/v1/auth/avatar', {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    })

    const result: ApiResponse<{ url: string }> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to upload avatar')
    }

    // Update stored user with new avatar
    const user = this.getStoredUser()
    if (user && result.data) {
      this.setStoredUser({ ...user, avatar: result.data.url })
    }

    return result.data!
  },

  // Get bank cards
  async getBankCards(): Promise<BankCard[]> {
    const response = await fetch('/api/v1/auth/cards', {
      method: 'GET',
      headers: this.getAuthHeaders(),
    })

    const result: ApiResponse<BankCard[]> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to get bank cards')
    }

    return result.data || []
  },

  // Bind bank card
  async bindBankCard(data: BindCardRequest): Promise<BankCard> {
    const response = await fetch('/api/v1/auth/cards', {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(data),
    })

    const result: ApiResponse<BankCard> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to bind bank card')
    }

    return result.data!
  },

  // Delete bank card
  async deleteBankCard(cardId: string): Promise<boolean> {
    const response = await fetch(`/api/v1/auth/cards/${cardId}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    })

    const result: ApiResponse<{ success: boolean }> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to delete bank card')
    }

    return result.data?.success || false
  },

  // Send verification code
  async sendCode(email: string, codeType: string): Promise<{ success: boolean; message: string; dev_code?: string }> {
    const response = await fetch('/api/v1/auth/code/send', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, code_type: codeType }),
    })

    const result = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to send verification code')
    }

    // 后端返回扁平格式，不是 ApiResponse 格式
    return {
      success: result.success,
      message: result.message,
      dev_code: result.dev_code,
    }
  },

  // Verify code
  async verifyCode(email: string, code: string, codeType: string): Promise<boolean> {
    const response = await fetch('/api/v1/auth/code/verify', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, code, code_type: codeType }),
    })

    const result = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Invalid verification code')
    }

    return true
  },

  // Third party login
  async thirdPartyLogin(provider: string, code: string): Promise<LoginResponse> {
    const response = await fetch('/api/v1/auth/third-party/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ provider, code }),
    })

    const result: ApiResponse<LoginResponse> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Third party login failed')
    }

    if (result.data) {
      this.setToken(result.data.token)
      this.setStoredUser(result.data.user)
    }

    return result.data!
  },

  // Get OAuth URL for third-party login
  async getOAuthUrl(provider: string): Promise<{ url: string; state: string }> {
    const response = await fetch(`/api/v1/auth/third-party/url/${provider}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    const result: ApiResponse<{ success: boolean; url: string; state: string }> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to get OAuth URL')
    }

    return { url: result.data!.url, state: result.data!.state }
  },

  // Third party bind
  async thirdPartyBind(provider: string, code: string, email: string, password: string): Promise<boolean> {
    const response = await fetch('/api/v1/auth/third-party/bind', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ provider, code, email, password }),
    })

    const result: ApiResponse<{ success: boolean; message: string }> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to bind account')
    }

    return true
  },

  // Reset password
  async resetPassword(email: string, code: string, newPassword: string): Promise<boolean> {
    const response = await fetch('/api/v1/auth/password/reset', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, code, new_password: newPassword }),
    })

    const result: ApiResponse<{ success: boolean; message: string }> = await response.json()

    if (!result.success) {
      throw new Error(result.error || 'Failed to reset password')
    }

    return true
  },
}

export default authClient
