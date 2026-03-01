import { useState } from 'react'
import { useNavigate, Link } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import { UserPlus, Mail, Lock, User, ArrowRight, MessageCircle, ArrowLeft } from 'lucide-react'

export function RegisterPage() {
  const navigate = useNavigate()
  const [username, setUsername] = useState('')
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [verificationCode, setVerificationCode] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isSendingCode, setIsSendingCode] = useState(false)
  const [codeSent, setCodeSent] = useState(false)
  const [error, setError] = useState('')

  const handleThirdPartyLogin = async (provider: string) => {
    setError('')
    try {
      const { url } = await authClient.getOAuthUrl(provider)
      window.location.href = url
    } catch (err) {
      const message = err instanceof Error ? err.message : '获取授权链接失败'
      setError(message)
    }
  }

  const handleSendCode = async () => {
    setError('')
    if (!email) {
      toast.error('请先输入邮箱')
      return
    }

    if (!email.includes('@')) {
      toast.error('请输入有效的邮箱地址')
      return
    }

    setIsSendingCode(true)
    try {
      const result = await authClient.sendCode(email, 'register')
      toast.success(result.message || '验证码已发送')
      setCodeSent(true)
    } catch (err) {
      const message = err instanceof Error ? err.message : '发送失败'
      setError(message)
    } finally {
      setIsSendingCode(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!username || !email || !password || !confirmPassword) {
      toast.error('请填写所有字段')
      return
    }

    if (password !== confirmPassword) {
      toast.error('两次输入的密码不一致')
      return
    }

    if (password.length < 6) {
      toast.error('密码长度至少为6位')
      return
    }

    if (username.length < 3 || username.length > 30) {
      toast.error('用户名长度需要在3-30个字符之间')
      return
    }

    // Verify code if sent
    if (codeSent && verificationCode) {
      try {
        await authClient.verifyCode(email, verificationCode, 'register')
      } catch (err) {
        const message = err instanceof Error ? err.message : '验证码验证失败'
        setError(message)
        return
      }
    }

    setIsLoading(true)
    try {
      await authClient.register({ username, email, password })
      toast.success('注册成功')
      navigate('/')
    } catch (err) {
      const message = err instanceof Error ? err.message : '注册失败'
      setError(message)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex flex-col bg-gradient-to-br from-blue-50 to-indigo-100 px-4 py-4">
      {/* Back button */}
      <div className="flex items-center">
        <button
          onClick={() => navigate(-1)}
          className="flex items-center text-gray-500 hover:text-gray-700 text-sm"
        >
          <ArrowLeft className="w-4 h-4 mr-1" />
          返回
        </button>
      </div>

      <div className="flex-1 flex items-center justify-center">
        <div className="max-w-md w-full">
          {/* Logo / Title */}
          <div className="text-center mb-4">
            <h1 className="text-3xl font-bold text-gray-900">AI Screen Code</h1>
            <p className="text-gray-600 text-sm">上传UI设计图片，AI自动生成前端代码</p>
          </div>

          <Card padding="md">
            <div className="flex items-center justify-center mb-4">
              <div className="p-2 bg-green-100 rounded-full">
                <UserPlus className="w-5 h-5 text-green-600" />
              </div>
            </div>

            <h2 className="text-lg font-semibold text-center mb-4">注册账号</h2>

            <form onSubmit={handleSubmit} className="space-y-3">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  用户名
                </label>
                <div className="relative">
                  <User className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                  <Input
                    type="text"
                    placeholder="3-30字符"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    className="pl-9 py-2"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  邮箱
                </label>
                <div className="relative">
                  <Mail className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                  <Input
                    type="email"
                    placeholder="your@email.com"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    className="pl-9 py-2"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  验证码
                </label>
                <div className="flex gap-2">
                  <Input
                    type="text"
                    placeholder="请输入验证码"
                    value={verificationCode}
                    onChange={(e) => setVerificationCode(e.target.value)}
                    className="py-2"
                    disabled={isLoading || isSendingCode}
                  />
                  <Button
                    type="button"
                    variant="outline"
                    onClick={handleSendCode}
                    disabled={isSendingCode || !email}
                    className="whitespace-nowrap text-xs px-2"
                  >
                    {isSendingCode ? '发送中' : codeSent ? '重发' : '发送验证码'}
                  </Button>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  密码
                </label>
                <div className="relative">
                  <Lock className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                  <Input
                    type="password"
                    placeholder="至少6位"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className="pl-9 py-2"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  确认密码
                </label>
                <div className="relative">
                  <Lock className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                  <Input
                    type="password"
                    placeholder="再次输入密码"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    className="pl-9 py-2"
                    disabled={isLoading}
                  />
                </div>
              </div>

              {error && (
                <div className="text-red-500 text-sm text-center bg-red-50 py-2 rounded-md">
                  {error}
                </div>
              )}

              <Button
                type="submit"
                isLoading={isLoading}
                className="w-full"
              >
                注册
                <ArrowRight className="w-4 h-4 ml-2" />
              </Button>
            </form>

            {/* Third party login */}
            <div className="mt-4">
              <div className="relative">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-300" />
                </div>
                <div className="relative flex justify-center text-xs">
                  <span className="px-2 bg-white text-gray-500">或者使用以下方式注册</span>
                </div>
              </div>

              <div className="mt-3 grid grid-cols-3 gap-2">
                <button
                  type="button"
                  onClick={() => handleThirdPartyLogin('qq')}
                  className="flex justify-center items-center py-2 border border-gray-300 rounded-md bg-white text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  <MessageCircle className="w-4 h-4 mr-1" />
                  QQ
                </button>
                <button
                  type="button"
                  onClick={() => handleThirdPartyLogin('wechat')}
                  className="flex justify-center items-center py-2 border border-gray-300 rounded-md bg-white text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  <MessageCircle className="w-4 h-4 mr-1" />
                  微信
                </button>
                <button
                  type="button"
                  onClick={() => handleThirdPartyLogin('douyin')}
                  className="flex justify-center items-center py-2 border border-gray-300 rounded-md bg-white text-sm font-medium text-gray-700 hover:bg-gray-50"
                >
                  <MessageCircle className="w-4 h-4 mr-1" />
                  抖音
                </button>
              </div>
            </div>

            <div className="mt-4 text-center">
              <p className="text-gray-600 text-sm">
                已有账号？{' '}
                <Link
                  to="/login"
                  className="text-blue-600 hover:text-blue-700 font-medium"
                >
                  立即登录
                </Link>
              </p>
            </div>
          </Card>
        </div>
      </div>
    </div>
  )
}
