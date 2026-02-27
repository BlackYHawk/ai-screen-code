import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import authClient from '@/api/auth'
import { Card, Button, Input } from '@/components/common'
import { toast } from 'sonner'
import { CreditCard, Plus, Trash2, ArrowLeft } from 'lucide-react'
import type { BankCard as BankCardType, BindCardRequest } from '@/types/api'

export function CardsPage() {
  const [cards, setCards] = useState<BankCardType[]>([])
  const [loading, setLoading] = useState(true)
  const [showAddForm, setShowAddForm] = useState(false)
  const [adding, setAdding] = useState(false)

  // Form state
  const [cardNumber, setCardNumber] = useState('')
  const [bankName, setBankName] = useState('')
  const [cardHolderName, setCardHolderName] = useState('')

  useEffect(() => {
    loadCards()
  }, [])

  const loadCards = async () => {
    try {
      const data = await authClient.getBankCards()
      setCards(data)
    } catch (err) {
      toast.error('加载银行卡列表失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleAddCard = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!cardNumber || !bankName || !cardHolderName) {
      toast.error('请填写所有字段')
      return
    }

    setAdding(true)
    try {
      const newCard = await authClient.bindBankCard({
        card_number: cardNumber,
        bank_name: bankName,
        card_holder_name: cardHolderName,
      } as BindCardRequest)
      setCards([newCard, ...cards])
      toast.success('绑定成功')
      resetForm()
      setShowAddForm(false)
    } catch (err) {
      toast.error(err instanceof Error ? err.message : '绑定失败')
    } finally {
      setAdding(false)
    }
  }

  const handleDeleteCard = async (cardId: string) => {
    if (!confirm('确定要解绑这张银行卡吗？')) {
      return
    }

    try {
      await authClient.deleteBankCard(cardId)
      setCards(cards.filter((c) => c.id !== cardId))
      toast.success('解绑成功')
    } catch (err) {
      toast.error('解绑失败')
      console.error(err)
    }
  }

  const resetForm = () => {
    setCardNumber('')
    setBankName('')
    setCardHolderName('')
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-2xl mx-auto px-4">
        {/* Header */}
        <div className="flex items-center mb-6">
          <Link
            to="/profile"
            className="p-2 hover:bg-gray-200 rounded-lg mr-2"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </Link>
          <h1 className="text-2xl font-bold text-gray-900">银行卡管理</h1>
        </div>

        {loading ? (
          <div className="text-center py-8 text-gray-500">加载中...</div>
        ) : (
          <div className="space-y-6">
            {/* Cards List */}
            <div className="space-y-4">
              {cards.length === 0 ? (
                <Card padding="lg" className="text-center">
                  <CreditCard className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                  <p className="text-gray-500">暂未绑定银行卡</p>
                  <p className="text-sm text-gray-400 mt-1">
                    点击下方按钮添加银行卡
                  </p>
                </Card>
              ) : (
                cards.map((card) => (
                  <Card key={card.id} padding="lg">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <div className="p-3 bg-blue-100 rounded-lg">
                          <CreditCard className="w-6 h-6 text-blue-600" />
                        </div>
                        <div className="ml-4">
                          <div className="font-medium text-gray-900">
                            {card.bank_name}
                          </div>
                          <div className="text-sm text-gray-500">
                            **** **** **** {card.card_number_last4}
                          </div>
                          <div className="text-xs text-gray-400">
                            持卡人: {card.card_holder_name}
                          </div>
                        </div>
                      </div>
                      <button
                        onClick={() => handleDeleteCard(card.id)}
                        className="p-2 text-red-500 hover:bg-red-50 rounded-lg"
                      >
                        <Trash2 className="w-5 h-5" />
                      </button>
                    </div>
                  </Card>
                ))
              )}
            </div>

            {/* Add Card Form */}
            {showAddForm ? (
              <Card padding="lg">
                <h2 className="text-lg font-semibold mb-4">绑定新银行卡</h2>
                <form onSubmit={handleAddCard} className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      银行卡号
                    </label>
                    <Input
                      value={cardNumber}
                      onChange={(e) => setCardNumber(e.target.value)}
                      placeholder="请输入银行卡号"
                      maxLength={19}
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      开户银行
                    </label>
                    <Input
                      value={bankName}
                      onChange={(e) => setBankName(e.target.value)}
                      placeholder="如：中国工商银行"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      持卡人姓名
                    </label>
                    <Input
                      value={cardHolderName}
                      onChange={(e) => setCardHolderName(e.target.value)}
                      placeholder="请输入持卡人姓名"
                    />
                  </div>

                  <div className="flex space-x-3">
                    <Button
                      type="submit"
                      isLoading={adding}
                      className="flex-1"
                    >
                      <Plus className="w-4 h-4 mr-2" />
                      绑定
                    </Button>
                    <Button
                      type="button"
                      variant="outline"
                      onClick={() => {
                        resetForm()
                        setShowAddForm(false)
                      }}
                    >
                      取消
                    </Button>
                  </div>
                </form>
              </Card>
            ) : (
              <Button
                onClick={() => setShowAddForm(true)}
                className="w-full"
              >
                <Plus className="w-4 h-4 mr-2" />
                添加银行卡
              </Button>
            )}
          </div>
        )}
      </div>
    </div>
  )
}

export default CardsPage
