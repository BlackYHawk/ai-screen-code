import { useNavigate } from 'react-router-dom'

export function SubscriptionBanner() {
  const navigate = useNavigate()

  return (
    <div
      className="mb-6 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg py-3 px-4 overflow-hidden cursor-pointer hover:from-blue-100 hover:to-indigo-100 transition-colors"
      onClick={() => navigate('/subscribe')}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          navigate('/subscribe')
        }
      }}
      aria-label="查看订阅优惠"
    >
      <div className="flex items-center animate-marquee whitespace-nowrap">
        <span className="text-blue-600 font-medium">🔥 专业版限时优惠中，首月仅需9.9元</span>
        <span className="mx-4 text-gray-400">|</span>
        <span className="text-purple-600 font-medium">🎉 新用户专享，订阅即送100积分</span>
        <span className="mx-4 text-gray-400">|</span>
        <span className="text-green-600 font-medium">🚀 企业版上线，定制AI解决方案</span>
        <span className="mx-4 text-gray-400">|</span>
        <span className="text-blue-600 font-medium">🔥 专业版限时优惠中，首月仅需9.9元</span>
        <span className="mx-4 text-gray-400">|</span>
        <span className="text-purple-600 font-medium">🎉 新用户专享，订阅即送100积分</span>
        <span className="mx-4 text-gray-400">|</span>
        <span className="text-green-600 font-medium">🚀 企业版上线，定制AI解决方案</span>
      </div>
    </div>
  )
}
