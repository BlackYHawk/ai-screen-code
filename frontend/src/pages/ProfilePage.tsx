import { useEffect, useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import { User as UserIcon, Save, CreditCard, LogOut } from 'lucide-react'
import type { User as UserType } from '@/types/api'

export function ProfilePage() {
  const navigate = useNavigate()
  const [user, setUser] = useState<UserType | null>(null)
  const [nickname, setNickname] = useState('')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    loadProfile()
  }, [])

  const loadProfile = async () => {
    try {
      const userData = await authClient.getProfile()
      setUser(userData)
      setNickname(userData.nickname || '')
    } catch (err) {
      toast.error('加载个人资料失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleSave = async () => {
    setSaving(true)
    try {
      const updated = await authClient.updateProfile({
        nickname: nickname || undefined,
      })
      setUser(updated)
      toast.success('保存成功')
    } catch (err) {
      toast.error('保存失败')
      console.error(err)
    } finally {
      setSaving(false)
    }
  }

  const handleLogout = () => {
    authClient.logout()
    toast.success('已退出登录')
    navigate('/')
  }

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-gray-500">加载中...</div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-2xl mx-auto px-4">
        <h1 className="text-2xl font-bold text-gray-900 mb-6">个人中心</h1>

        <div className="space-y-6">
          {/* User Info */}
          <Card padding="lg">
            <div className="flex items-center mb-6">
              <div className="p-3 bg-blue-100 rounded-full">
                <UserIcon className="w-6 h-6 text-blue-600" />
              </div>
              <h2 className="text-lg font-semibold ml-3">基本信息</h2>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  用户名
                </label>
                <Input
                  value={user?.username || ''}
                  disabled
                  className="bg-gray-100"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  邮箱
                </label>
                <Input
                  value={user?.email || ''}
                  disabled
                  className="bg-gray-100"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  昵称
                </label>
                <Input
                  value={nickname}
                  onChange={(e) => setNickname(e.target.value)}
                  placeholder="请输入昵称"
                />
              </div>

              <Button
                onClick={handleSave}
                isLoading={saving}
                className="w-full"
              >
                <Save className="w-4 h-4 mr-2" />
                保存修改
              </Button>
            </div>
          </Card>

          {/* Bank Cards */}
          <Card padding="lg">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center">
                <div className="p-3 bg-green-100 rounded-full">
                  <CreditCard className="w-6 h-6 text-green-600" />
                </div>
                <h2 className="text-lg font-semibold ml-3">银行卡管理</h2>
              </div>
              <Link
                to="/cards"
                className="text-blue-600 hover:text-blue-700 text-sm font-medium"
              >
                管理银行卡
              </Link>
            </div>
            <p className="text-gray-600 text-sm">
              点击"管理银行卡"添加或删除您的支付方式
            </p>
          </Card>

          {/* Logout */}
          <Button
            onClick={handleLogout}
            variant="outline"
            className="w-full"
          >
            <LogOut className="w-4 h-4 mr-2" />
            退出登录
          </Button>
        </div>
      </div>
    </div>
  )
}

export default ProfilePage
