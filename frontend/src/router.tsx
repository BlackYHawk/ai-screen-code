import { createBrowserRouter, Navigate } from 'react-router-dom'
import {
  HomePage,
  GeneratePage,
  PreviewPage,
  HistoryPage,
  SettingsPage,
  LoginPage,
  RegisterPage,
  ProfilePage,
  CardsPage,
  OrdersPage,
  SubscribePage,
  ForgotPasswordPage,
  OAuthCallbackPage,
} from './pages'

// Protected route wrapper
function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const token = localStorage.getItem('auth_token')
  if (!token) {
    return <Navigate to="/login" replace />
  }
  return <>{children}</>
}

export const router = createBrowserRouter([
  {
    path: '/',
    element: <HomePage />,
  },
  {
    path: '/generate',
    element: <GeneratePage />,
  },
  {
    path: '/preview/:id',
    element: <PreviewPage />,
  },
  {
    path: '/history',
    element: <HistoryPage />,
  },
  {
    path: '/settings',
    element: <SettingsPage />,
  },
  {
    path: '/login',
    element: <LoginPage />,
  },
  {
    path: '/register',
    element: <RegisterPage />,
  },
  {
    path: '/forgot-password',
    element: <ForgotPasswordPage />,
  },
  {
    path: '/auth/callback/:provider',
    element: <OAuthCallbackPage />,
  },
  {
    path: '/profile',
    element: (
      <ProtectedRoute>
        <ProfilePage />
      </ProtectedRoute>
    ),
  },
  {
    path: '/cards',
    element: (
      <ProtectedRoute>
        <CardsPage />
      </ProtectedRoute>
    ),
  },
  {
    path: '/orders',
    element: (
      <ProtectedRoute>
        <OrdersPage />
      </ProtectedRoute>
    ),
  },
  {
    path: '/subscribe',
    element: (
      <ProtectedRoute>
        <SubscribePage />
      </ProtectedRoute>
    ),
  },
])
