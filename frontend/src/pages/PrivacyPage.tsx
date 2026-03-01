import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Shield, Eye, Lock, User } from 'lucide-react'

export function PrivacyPage() {
  const navigate = useNavigate()

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50 py-8">
      <div className="max-w-3xl mx-auto px-4">
        {/* Header */}
        <div className="flex items-center mb-8">
          <button
            onClick={() => navigate(-1)}
            className="flex items-center justify-center w-10 h-10 rounded-xl bg-white shadow-sm hover:shadow-md transition-shadow mr-4"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <h1 className="text-2xl font-bold text-gray-900">隐私政策</h1>
        </div>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 space-y-6">
          {/* Payment Privacy Notice */}
          <div className="bg-gradient-to-r from-green-50 to-blue-50 rounded-xl p-5 border border-green-100">
            <div className="flex items-start gap-3">
              <div className="w-10 h-10 bg-green-100 rounded-xl flex items-center justify-center flex-shrink-0">
                <Lock className="w-5 h-5 text-green-600" />
              </div>
              <div>
                <h3 className="font-semibold text-gray-900 mb-1">支付信息保护</h3>
                <p className="text-sm text-gray-600">
                  我们高度重视您的支付隐私。所有支付信息由正规支付渠道处理，我们不会存储您的银行卡号、支付密码等敏感信息。
                </p>
              </div>
            </div>
          </div>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">一、信息收集</h2>
            <div className="space-y-3 text-gray-600 text-sm leading-relaxed">
              <div className="flex items-start gap-2">
                <User className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>账户信息：</strong>包括用户名、邮箱、手机号等用于账户创建和管理的基本信息。</p>
              </div>
              <div className="flex items-start gap-2">
                <Eye className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>使用数据：</strong>包括您使用本服务的行为数据、偏好设置等，用于优化服务体验。</p>
              </div>
              <div className="flex items-start gap-2">
                <Shield className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>支付信息：</strong>我们不存储您的完整银行卡号、支付密码等敏感信息，所有支付通过第三方安全渠道完成。</p>
              </div>
            </div>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">二、信息使用</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>提供、维护和改进我们的服务</li>
              <li>处理您的订阅和支付事务</li>
              <li>向您发送服务通知和更新</li>
              <li>响应您的客户服务请求</li>
              <li>遵守法律法规要求</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">三、信息保护</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>采用行业标准的安全加密技术保护数据传输</li>
              <li>严格限制员工访问用户信息的权限</li>
              <li>定期审查和更新安全措施</li>
              <li>发生安全事件时及时通知用户并采取补救措施</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">四、信息共享</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              除非法律要求或您同意，我们不会向第三方出售、交易或转让您的个人信息。
              为提供服务所需，我们可能与以下第三方共享信息：
            </p>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2 mt-2">
              <li>支付服务提供商（仅用于处理支付）</li>
              <li>云服务提供商（用于数据存储）</li>
              <li>法律执法机构（依法要求时）</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">五、您的权利</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>访问和查看您的个人信息</li>
              <li>更正不准确的信息</li>
              <li>删除您的个人信息</li>
              <li>导出您的数据</li>
              <li>取消订阅营销通讯</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">六、未成年人保护</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              我们的服务面向成年人。不满18周岁的未成年人应在父母或监护人指导下使用本服务。
              如发现未成年人未经授权使用服务，我们将立即删除相关信息。
            </p>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">七、联系我们</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              如您对隐私政策有任何疑问，或希望行使您的权利，请联系我们的客服团队。
            </p>
          </section>

          <div className="pt-4 border-t border-gray-100">
            <p className="text-xs text-gray-400 text-center">
              最后更新日期：2026年3月1日
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
