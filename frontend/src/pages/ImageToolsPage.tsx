import { useState, useCallback } from 'react'
import { Layout } from '@/components/layout'
import { Card, Button } from '@/components/common'
import { toast } from 'sonner'
import {
  Image,
  Type,
  User,
  Upload,
  X,
  Download,
  Loader2,
} from 'lucide-react'
import {
  compress,
  watermark,
  createPortrait,
  toDataUrl,
  getMimeType,
  getExtension,
  downloadData,
  initWasm,
} from '@/utils/imageProcessor'
import type {
  CompressionLevel,
  OutputFormat,
  WatermarkPosition,
  BackgroundColor,
  PhotoSize,
} from '@/types/image-wasm'

type TabType = 'compress' | 'watermark' | 'portrait'

export function ImageToolsPage() {
  const [activeTab, setActiveTab] = useState<TabType>('compress')
  const [currentImageData, setCurrentImageData] = useState<Uint8Array | null>(null)
  const [currentFormat, setCurrentFormat] = useState<OutputFormat>('png')
  const [resultData, setResultData] = useState<Uint8Array | null>(null)
  const [resultFormat, setResultFormat] = useState<OutputFormat>('png')
  const [isProcessing, setIsProcessing] = useState(false)
  const [compressionDone, setCompressionDone] = useState(false)

  const handleFileSelect = useCallback(async (file: File) => {
    const ext = file.name.split('.').pop()?.toLowerCase()
    let format: OutputFormat = 'png'
    if (ext === 'jpg' || ext === 'jpeg') format = 'jpeg'
    else if (['png', 'jpg', 'jpeg', 'webp', 'gif', 'bmp', 'tiff', 'tif'].includes(ext || '')) {
      format = ext as OutputFormat
    }

    setCurrentFormat(format)
    const buffer = await file.arrayBuffer()
    setCurrentImageData(new Uint8Array(buffer))
    setResultData(null)
    setCompressionDone(false)
  }, [])

  const handleCompress = useCallback(async () => {
    if (!currentImageData) return

    setIsProcessing(true)
    try {
      await initWasm()
      const level = (document.getElementById('compressLevel') as HTMLSelectElement).value as CompressionLevel
      const format = (document.getElementById('outputFormat') as HTMLSelectElement).value as OutputFormat
      const outputFormat = format === 'original' ? currentFormat : format

      const compressed = await compress(currentImageData, {
        level,
        format: outputFormat,
      })

      setResultData(compressed)
      setResultFormat(outputFormat)
      setCompressionDone(true)
      toast.success('压缩成功')
    } catch (error) {
      toast.error('压缩失败: ' + (error instanceof Error ? error.message : String(error)))
    } finally {
      setIsProcessing(false)
    }
  }, [currentImageData, currentFormat])

  const handleWatermark = useCallback(async () => {
    if (!currentImageData) return

    setIsProcessing(true)
    try {
      await initWasm()
      const text = (document.getElementById('watermarkText') as HTMLInputElement).value
      const fontSize = parseInt((document.getElementById('fontSize') as HTMLInputElement).value, 10)
      const position = (document.getElementById('watermarkPosition') as HTMLSelectElement).value as WatermarkPosition
      const rotation = parseInt((document.getElementById('rotation') as HTMLInputElement).value, 10)
      const spacing = parseInt((document.getElementById('watermarkSpacing') as HTMLInputElement).value, 10)
      const color = (document.getElementById('watermarkColor') as HTMLInputElement).value

      const watermarked = await watermark(currentImageData, {
        text,
        fontSize,
        position,
        rotation,
        spacing,
        color,
      })

      setResultData(watermarked)
      setResultFormat('png')
      toast.success('水印添加成功')
    } catch (error) {
      toast.error('添加水印失败: ' + (error instanceof Error ? error.message : String(error)))
    } finally {
      setIsProcessing(false)
    }
  }, [currentImageData])

  const handlePortrait = useCallback(async () => {
    if (!currentImageData) return

    setIsProcessing(true)
    try {
      await initWasm()
      const background = (document.getElementById('bgColor') as HTMLSelectElement).value as BackgroundColor
      const size = (document.getElementById('photoSize') as HTMLSelectElement).value as PhotoSize

      const portrait = await createPortrait(currentImageData, {
        background,
        size,
      })

      setResultData(portrait)
      setResultFormat('png')
      toast.success('证件照生成成功')
    } catch (error) {
      toast.error('生成证件照失败: ' + (error instanceof Error ? error.message : String(error)))
    } finally {
      setIsProcessing(false)
    }
  }, [currentImageData])

  const handleDownload = useCallback(() => {
    if (!resultData) return
    const ext = getExtension(resultFormat)
    downloadData(resultData, `output.${ext}`, getMimeType(resultFormat))
  }, [resultData, resultFormat])

  const handleDelete = useCallback(() => {
    setCurrentImageData(null)
    setResultData(null)
    setCompressionDone(false)
  }, [])

  const previewUrl = currentImageData ? toDataUrl(currentImageData, getMimeType(currentFormat)) : null
  const resultUrl = resultData ? toDataUrl(resultData, getMimeType(resultFormat)) : null

  return (
    <Layout>
      <div className="container mx-auto px-4 py-8 max-w-4xl">
        <h1 className="text-3xl font-bold mb-8 text-gray-900">图片处理工具</h1>

        {/* Tabs */}
        <div className="flex border-b border-gray-200 mb-6">
          <button
            onClick={() => setActiveTab('compress')}
            className={`flex items-center gap-2 px-6 py-3 border-b-2 transition-colors ${
              activeTab === 'compress'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <Image className="w-4 h-4" />
            压缩
          </button>
          <button
            onClick={() => setActiveTab('watermark')}
            className={`flex items-center gap-2 px-6 py-3 border-b-2 transition-colors ${
              activeTab === 'watermark'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <Type className="w-4 h-4" />
            水印
          </button>
          <button
            onClick={() => setActiveTab('portrait')}
            className={`flex items-center gap-2 px-6 py-3 border-b-2 transition-colors ${
              activeTab === 'portrait'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <User className="w-4 h-4" />
            证件照
          </button>
        </div>

        {/* Upload Area */}
        {!currentImageData ? (
          <label className="border-2 border-dashed border-gray-300 rounded-lg p-12 flex flex-col items-center justify-center cursor-pointer hover:border-blue-400 transition-colors">
            <input
              type="file"
              accept="image/*"
              className="hidden"
              onChange={(e) => e.target.files?.[0] && handleFileSelect(e.target.files[0])}
            />
            <Upload className="w-12 h-12 text-gray-400 mb-4" />
            <p className="text-gray-600">点击或拖拽上传图片</p>
          </label>
        ) : (
          <div className="space-y-6">
            {/* Preview */}
            <Card className="p-4">
              <div className="flex items-center justify-between mb-4">
                <span className="text-sm font-medium text-gray-700">预览</span>
                <button
                  onClick={handleDelete}
                  className="p-1 hover:bg-gray-100 rounded-full transition-colors"
                >
                  <X className="w-5 h-5 text-gray-500" />
                </button>
              </div>
              <img
                src={previewUrl || ''}
                alt="Preview"
                className="max-w-full max-h-64 mx-auto rounded-lg"
              />
            </Card>

            {/* Compress Tab */}
            {activeTab === 'compress' && (
              <Card className="p-6 space-y-4">
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">压缩级别</label>
                    <select
                      id="compressLevel"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    >
                      <option value="light">轻度</option>
                      <option value="normal">普通</option>
                      <option value="strong">强力</option>
                      <option value="extreme">极致</option>
                    </select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">输出格式</label>
                    <select
                      id="outputFormat"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    >
                      <option value="original">保持原格式</option>
                      <option value="jpeg">JPEG</option>
                      <option value="png">PNG</option>
                      <option value="webp">WebP</option>
                    </select>
                  </div>
                  <Button
                    onClick={handleCompress}
                    disabled={isProcessing || compressionDone}
                    className="w-full"
                  >
                    {isProcessing ? (
                      <>
                        <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                        处理中...
                      </>
                    ) : compressionDone ? (
                      '已完成'
                    ) : (
                      '压缩图片'
                    )}
                  </Button>
                </div>
              </Card>
            )}

            {/* Watermark Tab */}
            {activeTab === 'watermark' && (
              <Card className="p-6 space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">水印文本</label>
                  <input
                    type="text"
                    id="watermarkText"
                    placeholder="输入水印文字"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">字体大小</label>
                    <input
                      type="number"
                      id="fontSize"
                      defaultValue={24}
                      min={8}
                      max={200}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">位置</label>
                    <select
                      id="watermarkPosition"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    >
                      <option value="top-left">左上角</option>
                      <option value="top-center">上中</option>
                      <option value="top-right">右上角</option>
                      <option value="center-left">左中</option>
                      <option value="center" selected>中心</option>
                      <option value="center-right">右中</option>
                      <option value="bottom-left">左下角</option>
                      <option value="bottom-center">下中</option>
                      <option value="bottom-right">右下角</option>
                    </select>
                  </div>
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">旋转角度</label>
                    <input
                      type="number"
                      id="rotation"
                      defaultValue={0}
                      min={0}
                      max={360}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">间距</label>
                    <input
                      type="number"
                      id="watermarkSpacing"
                      defaultValue={100}
                      min={0}
                      max={500}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    />
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">颜色</label>
                  <div className="flex items-center gap-3">
                    <input
                      type="color"
                      id="watermarkColor"
                      defaultValue="#000000"
                      className="w-12 h-10 p-0 border border-gray-300 rounded cursor-pointer"
                    />
                    <span className="text-sm text-gray-500">选择水印颜色</span>
                  </div>
                </div>
                <Button
                  onClick={handleWatermark}
                  disabled={isProcessing}
                  className="w-full"
                >
                  {isProcessing ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      处理中...
                    </>
                  ) : (
                    '添加水印'
                  )}
                </Button>
              </Card>
            )}

            {/* Portrait Tab */}
            {activeTab === 'portrait' && (
              <Card className="p-6 space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">底色</label>
                  <select
                    id="bgColor"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="white">白底</option>
                    <option value="red">红底</option>
                    <option value="blue">蓝底</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">尺寸</label>
                  <select
                    id="photoSize"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="one-inch">一寸 (25x35mm)</option>
                    <option value="one-inch-small">小一寸 (22x32mm)</option>
                    <option value="one-inch-large">大一寸 (33x48mm)</option>
                    <option value="two-inch">二寸 (35x49mm)</option>
                    <option value="two-inch-small">小二寸 (35x45mm)</option>
                    <option value="two-inch-large">大二寸 (35x53mm)</option>
                    <option value="three-inch">三寸 (58x84mm)</option>
                    <option value="four-inch">四寸 (76x102mm)</option>
                    <option value="five-inch">五寸 (89x127mm)</option>
                  </select>
                </div>
                <Button
                  onClick={handlePortrait}
                  disabled={isProcessing}
                  className="w-full"
                >
                  {isProcessing ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      处理中...
                    </>
                  ) : (
                    '生成证件照'
                  )}
                </Button>
              </Card>
            )}

            {/* Result */}
            {resultUrl && (
              <Card className="p-6 space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium text-gray-700">处理结果</span>
                  <Button onClick={handleDownload} variant="outline" size="sm">
                    <Download className="w-4 h-4 mr-2" />
                    下载图片
                  </Button>
                </div>
                <img
                  src={resultUrl}
                  alt="Result"
                  className="max-w-full mx-auto rounded-lg"
                />
              </Card>
            )}
          </div>
        )}
      </div>
    </Layout>
  )
}
