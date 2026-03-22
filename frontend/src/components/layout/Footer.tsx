import { Link } from 'react-router-dom'
import { useState, useEffect } from 'react'

// Subscription banner data
const bannerData = [
  { id: 1, title: '升级订阅，解锁更多功能', desc: '专业版限时优惠中', color: 'from-blue-500 to-blue-600' },
  { id: 2, title: '新用户专享', desc: '首月仅需9.9元', color: 'from-purple-500 to-purple-600' },
  { id: 3, title: '企业版上线', desc: '定制化AI解决方案', color: 'from-green-500 to-green-600' },
]

export function Footer() {
  const [currentBanner, setCurrentBanner] = useState(0)

  // Auto-rotate banner
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentBanner((prev) => (prev + 1) % bannerData.length)
    }, 3000)
    return () => clearInterval(timer)
  }, [])

  return (
    <>
      {/* Subscription Banner Carousel - Above Footer */}
      <div className="relative overflow-hidden">
        <div
          className={`bg-gradient-to-r ${bannerData[currentBanner].color} py-3 transition-all duration-500`}
        >
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex items-center justify-center space-x-6">
            <div className="text-white text-center">
              <p className="font-semibold">{bannerData[currentBanner].title}</p>
              <p className="text-sm text-white/80">{bannerData[currentBanner].desc}</p>
            </div>
            <Link
              to="/subscribe"
              className="px-4 py-2 bg-white text-blue-600 rounded-lg font-medium hover:bg-blue-50 transition-colors whitespace-nowrap"
            >
              立即订阅
            </Link>
          </div>
        </div>
        {/* Banner indicators */}
        <div className="absolute bottom-1 left-1/2 transform -translate-x-1/2 flex space-x-1">
          {bannerData.map((_, idx) => (
            <button
              key={idx}
              onClick={() => setCurrentBanner(idx)}
              className={`w-1.5 h-1.5 rounded-full transition-colors ${
                idx === currentBanner ? 'bg-white' : 'bg-white/50'
              }`}
            />
          ))}
        </div>
      </div>

      <footer className="bg-white border-t border-gray-200 py-4">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <p className="text-center text-sm text-gray-500">
            AI Image - 专业图片处理工具
          </p>
        </div>
      </footer>
    </>
  )
}
