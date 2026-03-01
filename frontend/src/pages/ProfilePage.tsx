import { useEffect, useRef, useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import {
  User as UserIcon,
  Save,
  CreditCard,
  LogOut,
  ArrowLeft,
  Settings as SettingsIcon,
  Receipt,
  Camera,
} from 'lucide-react'
import type { User as UserType } from '@/types/api'

// Compress image to target size (100KB)
function compressImage(
  file: File,
  maxSizeKB: number = 100,
  maxWidth: number = 400
): Promise<File> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const img = new Image()
      img.onload = () => {
        const canvas = document.createElement('canvas')
        let width = img.width
        let height = img.height

        // Resize if needed
        if (width > maxWidth) {
          height = (height * maxWidth) / width
          width = maxWidth
        }

        canvas.width = width
        canvas.height = height
        const ctx = canvas.getContext('2d')
        if (!ctx) {
          reject(new Error('Failed to get canvas context'))
          return
        }

        ctx.drawImage(img, 0, 0, width, height)

        // Try to compress to target size
        let quality = 0.9
        const maxBytes = maxSizeKB * 1024

        const tryCompress = () => {
          const dataUrl = canvas.toDataURL('image/jpeg', quality)
          const base64 = dataUrl.split(',')[1]
          const bytes = (base64.length * 3) / 4

          if (bytes <= maxBytes || quality <= 0.1) {
            // Convert back to file
            const binary = atob(base64)
            const array = new Uint8Array(binary.length)
            for (let i = 0; i < binary.length; i++) {
              array[i] = binary.charCodeAt(i)
            }
            const compressedFile = new File([array], file.name.replace(/\.[^.]+$/, '.jpg'), {
              type: 'image/jpeg',
            })
            resolve(compressedFile)
          } else {
            quality -= 0.1
            tryCompress()
          }
        }

        tryCompress()
      }
      img.onerror = () => reject(new Error('Failed to load image'))
      img.src = e.target?.result as string
    }
    reader.onerror = () => reject(new Error('Failed to read file'))
    reader.readAsDataURL(file)
  })
}

export function ProfilePage() {
  const navigate = useNavigate()
  const [user, setUser] = useState<UserType | null>(null)
  const [nickname, setNickname] = useState('')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [isEditing, setIsEditing] = useState(false)
  const [uploading, setUploading] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const hasNickname = nickname.trim().length > 0

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
      setIsEditing(false)
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

  const handleEditClick = () => {
    setIsEditing(true)
  }

  const handleAvatarClick = () => {
    fileInputRef.current?.click()
  }

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return

    // Validate file type
    if (!['image/jpeg', 'image/png', 'image/gif', 'image/webp'].includes(file.type)) {
      toast.error('仅支持 jpg, png, gif, webp 格式')
      return
    }

    setUploading(true)
    try {
      // Compress image to max 100KB
      const compressedFile = await compressImage(file, 100, 400)

      // Upload to server
      const result = await authClient.uploadAvatar(compressedFile)

      // Update user with new avatar
      setUser((prev) => (prev ? { ...prev, avatar: result.url } : null))
      toast.success('头像上传成功')
    } catch (err) {
      toast.error('头像上传失败')
      console.error(err)
    } finally {
      setUploading(false)
      // Reset file input
      if (fileInputRef.current) {
        fileInputRef.current.value = ''
      }
    }
  }

  const handleCancelEdit = () => {
    setNickname(user?.nickname || '')
    setIsEditing(false)
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
        {/* 返回按钮 */}
        <div className="flex items-center mb-6">
          <Link to="/" className="p-2 hover:bg-gray-200 rounded-lg mr-2">
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </Link>
          <h1 className="text-2xl font-bold text-gray-900">个人中心</h1>
        </div>

        <div className="space-y-6">
          {/* User Info */}
          <Card padding="lg">
            {/* Header with edit button for no nickname */}
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center">
                {/* Avatar or icon */}
                {isEditing ? (
                  <div className="relative">
                    <button
                      onClick={handleAvatarClick}
                      disabled={uploading}
                      className="w-14 h-14 rounded-full bg-blue-100 flex items-center justify-center hover:bg-blue-200 transition-colors overflow-hidden"
                    >
                      {user?.avatar ? (
                        <img
                          src={user.avatar}
                          alt="头像"
                          className="w-full h-full object-cover"
                        />
                      ) : (
                        <UserIcon className="w-6 h-6 text-blue-600" />
                      )}
                      {uploading && (
                        <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center">
                          <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
                        </div>
                      )}
                    </button>
                    <div className="absolute -bottom-1 -right-1 w-6 h-6 bg-blue-600 rounded-full flex items-center justify-center">
                      <Camera className="w-3 h-3 text-white" />
                    </div>
                  </div>
                ) : (
                  <div className="p-3 bg-blue-100 rounded-full">
                    {user?.avatar ? (
                      <img
                        src={user.avatar}
                        alt="头像"
                        className="w-6 h-6 rounded-full object-cover"
                      />
                    ) : (
                      <UserIcon className="w-6 h-6 text-blue-600" />
                    )}
                  </div>
                )}
                <div className="ml-3">
                  <h2 className="text-lg font-semibold">基本信息</h2>
                  {hasNickname && !isEditing && (
                    <p className="text-sm text-gray-500">{nickname}</p>
                  )}
                </div>
              </div>
              {/* Edit button when no nickname */}
              {!hasNickname && !isEditing && (
                <button
                  onClick={handleEditClick}
                  className="px-3 py-1.5 text-sm text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                >
                  设置昵称
                </button>
              )}
              {isEditing && (
                <button
                  onClick={handleCancelEdit}
                  className="px-3 py-1.5 text-sm text-gray-500 hover:bg-gray-100 rounded-lg transition-colors"
                >
                  取消
                </button>
              )}
            </div>

            <div className="space-y-4">
              {/* Username */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">用户名</label>
                <Input value={user?.username || ''} disabled className="bg-gray-100" />
              </div>

              {/* Email */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">邮箱</label>
                <Input value={user?.email || ''} disabled className="bg-gray-100" />
              </div>

              {/* Nickname - show only when editing or has nickname */}
              {(isEditing || hasNickname) && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">昵称</label>
                  <Input
                    value={nickname}
                    onChange={(e) => setNickname(e.target.value)}
                    placeholder="请输入昵称"
                  />
                </div>
              )}

              {/* Save button when editing */}
              {isEditing && (
                <Button onClick={handleSave} isLoading={saving} className="w-full">
                  <Save className="w-4 h-4 mr-2" />
                  保存修改
                </Button>
              )}
            </div>

            {/* Hidden file input */}
            <input
              ref={fileInputRef}
              type="file"
              accept="image/jpeg,image/png,image/gif,image/webp"
              onChange={handleFileChange}
              className="hidden"
            />
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
            <p className="text-gray-600 text-sm">点击&quot;管理银行卡&quot;添加或删除您的支付方式</p>
          </Card>

          {/* My Subscription */}
          <Card padding="lg">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center">
                <div className="p-3 bg-orange-100 rounded-full">
                  <Receipt className="w-6 h-6 text-orange-600" />
                </div>
                <h2 className="text-lg font-semibold ml-3">我的订阅</h2>
              </div>
              <Link to="/orders" className="text-blue-600 hover:text-blue-700 text-sm font-medium">
                查看订单
              </Link>
            </div>
            <p className="text-gray-600 text-sm">查看您的订阅套餐和订单记录</p>
          </Card>

          {/* Model Settings */}
          <Card padding="lg">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center">
                <div className="p-3 bg-purple-100 rounded-full">
                  <SettingsIcon className="w-6 h-6 text-purple-600" />
                </div>
                <h2 className="text-lg font-semibold ml-3">模型设置</h2>
              </div>
              <Link
                to="/settings"
                className="text-blue-600 hover:text-blue-700 text-sm font-medium"
              >
                前往设置
              </Link>
            </div>
            <p className="text-gray-600 text-sm">配置 AI 模型 API Key 和默认模型</p>
          </Card>

          {/* Logout */}
          <Button onClick={handleLogout} variant="outline" className="w-full">
            <LogOut className="w-4 h-4 mr-2" />
            退出登录
          </Button>
        </div>
      </div>
    </div>
  )
}

export default ProfilePage
