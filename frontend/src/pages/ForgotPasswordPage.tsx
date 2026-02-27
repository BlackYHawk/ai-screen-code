import { useState } from 'react'
import { useNavigate, Link } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import { KeyRound, Mail, ArrowRight, Send } from 'lucide-react'

export function ForgotPasswordPage() {
  const navigate = useNavigate()
  const [email, setEmail] = useState('')
  const [verificationCode, setVerificationCode] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isSendingCode, setIsSendingCode] = useState(false)
  const [step, setStep] = useState<'email' | 'reset'>('email')

  const handleSendCode = async () => {
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
      const result = await authClient.sendCode(email, 'reset_password')
      toast.success(result.message || '验证码已发送')
      setStep('reset')
    } catch (err) {
      const message = err instanceof Error ? err.message : '发送失败'
      toast.error(message)
    } finally {
      setIsSendingCode(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!verificationCode || !newPassword || !confirmPassword) {
      toast.error('请填写所有字段')
      return
    }

    if (newPassword !== confirmPassword) {
      toast.error('两次输入的密码不一致')
      return
    }

    if (newPassword.length < 6) {
      toast.error('密码长度至少为6位')
      return
    }

    setIsLoading(true)
    try {
      await authClient.resetPassword(email, verificationCode, newPassword)
      toast.success('密码重置成功')
      navigate('/login')
    } catch (err) {
      const message = err instanceof Error ? err.message : '重置失败'
      toast.error(message)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 px-4">
      <div className="max-w-md w-full">
        {/* Logo / Title */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">AI Screen Code</h1>
          <p className="text-gray-600">上传UI设计图片，AI自动生成前端代码</p>
        </div>

        <Card padding="lg">
          <div className="flex items-center justify-center mb-6">
            <div className="p-3 bg-orange-100 rounded-full">
              <KeyRound className="w-6 h-6 text-orange-600" />
            </div>
          </div>

          <h2 className="text-xl font-semibold text-center mb-6">重置密码</h2>

          {step === 'email' ? (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  邮箱
                </label>
                <div className="relative">
                  <Mail className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                  <Input
                    type="email"
                    placeholder="your@email.com"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    className="pl-10"
                    disabled={isSendingCode}
                  />
                </div>
              </div>

              <Button
                onClick={handleSendCode}
                isLoading={isSendingCode}
                className="w-full"
                size="lg"
              >
                发送验证码
                <Send className="w-4 h-4 ml-2" />
              </Button>
            </div>
          ) : (
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  邮箱
                </label>
                <div className="relative">
                  <Mail className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                  <Input
                    type="email"
                    value={email}
                    className="pl-10"
                    disabled
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  验证码
                </label>
                <Input
                  type="text"
                  placeholder="请输入验证码"
                  value={verificationCode}
                  onChange={(e) => setVerificationCode(e.target.value)}
                  disabled={isLoading}
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  新密码
                </label>
                <div className="relative">
                  <KeyRound className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                  <Input
                    type="password"
                    placeholder="至少6位密码"
                    value={newPassword}
                    onChange={(e) => setNewPassword(e.target.value)}
                    className="pl-10"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  确认新密码
                </label>
                <div className="relative">
                  <KeyRound className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                  <Input
                    type="password"
                    placeholder="再次输入新密码"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    className="pl-10"
                    disabled={isLoading}
                  />
                </div>
              </div>

              <Button
                type="submit"
                isLoading={isLoading}
                className="w-full"
                size="lg"
              >
                重置密码
                <ArrowRight className="w-4 h-4 ml-2" />
              </Button>
            </form>
          )}

          <div className="mt-6 text-center">
            <p className="text-gray-600">
              想起密码了？{' '}
              <Link
                to="/login"
                className="text-blue-600 hover:text-blue-700 font-medium"
              >
                立即登录
              </Link>
            </p>
          </div>
        </Card>

        {/* Back to home */}
        <div className="mt-6 text-center">
          <Link
            to="/"
            className="text-gray-500 hover:text-gray-700 text-sm"
          >
            返回首页
          </Link>
        </div>
      </div>
    </div>
  )
}
