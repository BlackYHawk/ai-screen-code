import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Shield, Lock, CreditCard, AlertTriangle } from 'lucide-react'

export function TermsPage() {
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
          <h1 className="text-2xl font-bold text-gray-900">服务条款</h1>
        </div>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 space-y-6">
          {/* Payment Security Notice */}
          <div className="bg-gradient-to-r from-blue-50 to-purple-50 rounded-xl p-5 border border-blue-100">
            <div className="flex items-start gap-3">
              <div className="w-10 h-10 bg-blue-100 rounded-xl flex items-center justify-center flex-shrink-0">
                <Shield className="w-5 h-5 text-blue-600" />
              </div>
              <div>
                <h3 className="font-semibold text-gray-900 mb-1">支付安全保障</h3>
                <p className="text-sm text-gray-600">
                  我们承诺为您的支付安全保驾护航。所有支付流程均通过正规支付渠道完成，资金安全有保障。
                </p>
              </div>
            </div>
          </div>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">一、服务条款概述</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              欢迎使用 AI Image（以下简称"本服务"）。本服务条款是您与本服务提供者之间的协议，
              规定了您使用本服务的权利和义务。在您开始使用本服务前，请仔细阅读并理解本服务条款。
            </p>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">二、订阅与支付</h2>
            <div className="space-y-3 text-gray-600 text-sm leading-relaxed">
              <div className="flex items-start gap-2">
                <CreditCard className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>支付方式：</strong>我们支持微信支付、支付宝、云闪付等正规支付渠道。所有支付均通过安全加密通道处理。</p>
              </div>
              <div className="flex items-start gap-2">
                <Lock className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>资金安全：</strong>您的支付资金由正规支付机构托管，交易完成后直接转入商家账户，我们不会留存您的支付密码或银行卡信息。</p>
              </div>
              <div className="flex items-start gap-2">
                <AlertTriangle className="w-4 h-4 text-blue-500 mt-1 flex-shrink-0" />
                <p><strong>防骗提示：</strong>本服务不会通过任何非官方渠道索要您的支付密码或验证码。如遇可疑情况，请立即报警。</p>
              </div>
            </div>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">三、订阅周期与续费</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>订阅按月计费，自动续费直到您主动取消</li>
              <li>订阅费用将在每个计费周期首日自动扣除</li>
              <li>如续费失败，服务将暂停，请及时处理支付问题</li>
              <li>我们会在扣费前发送提醒通知</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">四、退款政策</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>新用户首次订阅后7天内可申请无理由退款</li>
              <li>因技术问题导致服务无法正常使用，可申请全额或部分退款</li>
              <li>退款申请将在3-5个工作日内处理完成</li>
              <li>超过7天的正常订阅不支持退款</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">五、用户责任</h2>
            <ul className="list-disc list-inside text-gray-600 text-sm leading-relaxed space-y-2">
              <li>妥善保管您的账户信息和支付凭证</li>
              <li>不使用本服务从事任何违法或违规活动</li>
              <li>不将账户转让、出租或借给他人使用</li>
              <li>发现异常支付行为立即联系我们</li>
            </ul>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">六、免责声明</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              本服务按"现状"提供，不提供任何明示或暗示的保证。因不可抗力或第三方原因导致的损失，
              我们不承担责任。用户应自行承担使用本服务的风险。
            </p>
          </section>

          <section>
            <h2 className="text-lg font-semibold text-gray-900 mb-3">七、联系我们</h2>
            <p className="text-gray-600 text-sm leading-relaxed">
              如您对服务条款有任何疑问，或需要帮助，请联系我们的客服团队。
              我们承诺在1-3个工作日内回复您的咨询。
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
