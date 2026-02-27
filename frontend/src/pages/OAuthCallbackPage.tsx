import { useEffect, useState } from 'react'
import { useSearchParams, useNavigate } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card } from '@/components/common'
import { toast } from 'sonner'
import { Loader2 } from 'lucide-react'

export function OAuthCallbackPage() {
  const [searchParams] = useSearchParams()
  const navigate = useNavigate()
  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading')
  const [errorMessage, setErrorMessage] = useState('')

  useEffect(() => {
    const handleCallback = async () => {
      // Get provider from URL path
      const path = window.location.pathname
      const providerMatch = path.match(/\/auth\/callback\/(\w+)/)
      const provider = providerMatch ? providerMatch[1] : 'unknown'

      // Get code from query params
      const code = searchParams.get('code')
      const error = searchParams.get('error')
      const errorDescription = searchParams.get('error_description')

      if (error) {
        setStatus('error')
        setErrorMessage(errorDescription || error)
        toast.error(`授权失败: ${errorDescription || error}`)
        setTimeout(() => navigate('/login'), 3000)
        return
      }

      if (!code) {
        setStatus('error')
        setErrorMessage('未获取到授权码')
        toast.error('未获取到授权码')
        setTimeout(() => navigate('/login'), 3000)
        return
      }

      try {
        // Attempt third-party login
        await authClient.thirdPartyLogin(provider, code)
        setStatus('success')
        toast.success('登录成功')
        navigate('/')
      } catch (err) {
        // If user not found, they need to bind their account
        const errorMsg = err instanceof Error ? err.message : '登录失败'
        if (errorMsg.includes('Please bind your account first') || errorMsg.includes('User not found')) {
          setStatus('error')
          setErrorMessage('请先绑定账号')
          toast.error('该第三方账号未绑定，请先绑定账号')
          // Store provider and code for binding page
          sessionStorage.setItem('oauth_pending_provider', provider)
          sessionStorage.setItem('oauth_pending_code', code)
          setTimeout(() => navigate('/bind-account'), 3000)
        } else {
          setStatus('error')
          setErrorMessage(errorMsg)
          toast.error(errorMsg)
          setTimeout(() => navigate('/login'), 3000)
        }
      }
    }

    handleCallback()
  }, [searchParams, navigate])

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 px-4">
      <Card padding="lg" className="max-w-md w-full text-center">
        {status === 'loading' && (
          <>
            <Loader2 className="w-12 h-12 mx-auto mb-4 text-blue-600 animate-spin" />
            <h2 className="text-xl font-semibold mb-2">正在处理授权...</h2>
            <p className="text-gray-600">请稍候，正在完成登录流程</p>
          </>
        )}

        {status === 'success' && (
          <>
            <div className="w-12 h-12 mx-auto mb-4 rounded-full bg-green-100 flex items-center justify-center">
              <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 className="text-xl font-semibold mb-2">登录成功</h2>
            <p className="text-gray-600">正在跳转到首页...</p>
          </>
        )}

        {status === 'error' && (
          <>
            <div className="w-12 h-12 mx-auto mb-4 rounded-full bg-red-100 flex items-center justify-center">
              <svg className="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
            <h2 className="text-xl font-semibold mb-2">授权失败</h2>
            <p className="text-gray-600">{errorMessage}</p>
            <p className="text-gray-500 text-sm mt-2">正在跳转到登录页...</p>
          </>
        )}
      </Card>
    </div>
  )
}
