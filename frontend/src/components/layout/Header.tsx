import { useEffect, useState } from 'react'
import { Link, useLocation, useNavigate } from 'react-router-dom'
import { Code2, History, Home, User, LogOut } from 'lucide-react'
import { WindowControls } from '@/components/common/WindowControls'
import authClient from '@/api/auth'
import type { User as UserType } from '@/types/api'

const navItems = [
  { path: '/', label: '首页', icon: Home },
  { path: '/history', label: '历史', icon: History },
]

export function Header() {
  const location = useLocation()
  const navigate = useNavigate()
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  const [user, setUser] = useState<UserType | null>(null)

  useEffect(() => {
    const checkAuth = () => {
      const token = authClient.getToken()
      const storedUser = authClient.getStoredUser()
      setIsLoggedIn(!!token)
      setUser(storedUser)
    }

    checkAuth()

    // Listen for storage changes
    window.addEventListener('storage', checkAuth)
    return () => window.removeEventListener('storage', checkAuth)
  }, [location.pathname])

  const handleLogout = () => {
    authClient.logout()
    setIsLoggedIn(false)
    setUser(null)
    navigate('/')
  }

  return (
    <header className="bg-white border-b border-gray-200 sticky top-0 z-40">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link to="/" className="flex items-center space-x-2">
            <Code2 className="w-8 h-8 text-blue-600" />
            <span className="text-xl font-bold text-gray-900">AI Screen Code</span>
          </Link>

          {/* Navigation */}
          <nav className="flex items-center space-x-1">
            {navItems.map((item) => {
              const Icon = item.icon
              const isActive = location.pathname === item.path
              return (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`
                    flex items-center space-x-2 px-3 py-2 rounded-lg
                    text-sm font-medium transition-colors
                    ${
                      isActive
                        ? 'bg-blue-50 text-blue-600'
                        : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                    }
                  `}
                >
                  <Icon className="w-4 h-4" />
                  <span>{item.label}</span>
                </Link>
              )
            })}
          </nav>

          {/* User Menu */}
          <div className="flex items-center space-x-2">
            {isLoggedIn ? (
              <>
                <Link
                  to="/profile"
                  className="flex items-center space-x-2 px-3 py-2 rounded-lg text-sm font-medium text-gray-600 hover:bg-gray-100"
                >
                  <User className="w-4 h-4" />
                  <span>{user?.nickname || user?.username || '个人中心'}</span>
                </Link>
                <button
                  onClick={handleLogout}
                  className="flex items-center space-x-2 px-3 py-2 rounded-lg text-sm font-medium text-gray-600 hover:bg-gray-100"
                >
                  <LogOut className="w-4 h-4" />
                </button>
              </>
            ) : (
              <>
                <Link
                  to="/login"
                  className="px-3 py-2 text-sm font-medium text-gray-600 hover:text-gray-900"
                >
                  登录
                </Link>
                <Link
                  to="/register"
                  className="px-3 py-2 text-sm font-medium bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                >
                  注册
                </Link>
              </>
            )}

            {/* Window Controls */}
            <WindowControls />
          </div>
        </div>
      </div>
    </header>
  )
}
