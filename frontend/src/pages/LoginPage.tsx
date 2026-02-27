import { useState } from 'react'
import { useNavigate, Link } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import { LogIn, Mail, Lock, ArrowRight, MessageCircle, Home } from 'lucide-react'

export function LoginPage() {
  const navigate = useNavigate()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [isLoading, setIsLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!email || !password) {
      toast.error('请填写所有字段')
      return
    }

    setIsLoading(true)
    try {
      await authClient.login({ email, password })
      toast.success('登录成功')
      navigate('/')
    } catch (err) {
      const message = err instanceof Error ? err.message : '登录失败'
      toast.error(message)
    } finally {
      setIsLoading(false)
    }
  }

  const handleThirdPartyLogin = async (provider: string) => {
    try {
      const { url } = await authClient.getOAuthUrl(provider)
      window.location.href = url
    } catch (err) {
      const message = err instanceof Error ? err.message : '获取授权链接失败'
      toast.error(message)
    }
  }

  return (
    <div className="min-h-screen flex flex-col bg-gradient-to-br from-blue-50 to-indigo-100 px-4 py-4">
      {/* Back to home - top right */}
      <div className="flex justify-end">
        <Link
          to="/"
          className="flex items-center text-gray-500 hover:text-gray-700 text-sm"
        >
          <Home className="w-4 h-4 mr-1" />
          返回首页
        </Link>
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
              <div className="p-2 bg-blue-100 rounded-full">
                <LogIn className="w-5 h-5 text-blue-600" />
              </div>
            </div>

            <h2 className="text-lg font-semibold text-center mb-4">登录账号</h2>

            <form onSubmit={handleSubmit} className="space-y-3">
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
                  密码
                </label>
                <div className="relative">
                  <Lock className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                  <Input
                    type="password"
                    placeholder="请输入密码"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className="pl-9 py-2"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <div className="flex justify-end">
                <Link
                  to="/forgot-password"
                  className="text-sm text-blue-600 hover:text-blue-700"
                >
                  忘记密码？
                </Link>
              </div>

              <Button
                type="submit"
                isLoading={isLoading}
                className="w-full"
              >
                登录
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
                  <span className="px-2 bg-white text-gray-500">或者使用以下方式登录</span>
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
                还没有账号？{' '}
                <Link
                  to="/register"
                  className="text-blue-600 hover:text-blue-700 font-medium"
                >
                  立即注册
                </Link>
              </p>
            </div>
          </Card>
        </div>
      </div>
    </div>
  )
}
