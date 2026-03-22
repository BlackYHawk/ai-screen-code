import { useState, useCallback } from 'react'
import { Download } from 'lucide-react'
import { watermark, toDataUrl, getMimeType } from '@/utils/imageProcessor'
import { Button } from '@/components/common'

const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10MB

interface WatermarkToolProps {
  onProcessed?: (url: string) => void
}

export function WatermarkTool({ onProcessed }: WatermarkToolProps) {
  const [file, setFile] = useState<File | null>(null)
  const [preview, setPreview] = useState<string | null>(null)
  const [watermarkText, setWatermarkText] = useState('')
  const [fontSize, setFontSize] = useState(48)
  const [fontWeight, setFontWeight] = useState<'normal' | 'bold'>('normal')
  const [rotation, setRotation] = useState(-30)
  const [watermarkColor, setWatermarkColor] = useState('#808080')
  const [spacing, setSpacing] = useState(10)
  const [processing, setProcessing] = useState(false)
  const [resultUrl, setResultUrl] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const validateFile = useCallback((file: File): string | null => {
    if (!['image/png', 'image/jpeg', 'image/webp'].includes(file.type)) {
      return '不支持的图片格式，请上传 PNG、JPG、JPEG 或 WebP 格式'
    }
    if (file.type === 'image/svg+xml' || file.name.toLowerCase().endsWith('.svg')) {
      return '不支持 SVG 格式'
    }
    if (file.size > MAX_FILE_SIZE) {
      return '图片大小不能超过 10MB'
    }
    return null
  }, [])

  const handleFileSelect = useCallback((selectedFile: File) => {
    const validationError = validateFile(selectedFile)
    if (validationError) {
      setError(validationError)
      return
    }
    setError(null)
    setFile(selectedFile)
    setResultUrl(null)

    const reader = new FileReader()
    reader.onload = (e) => {
      setPreview(e.target?.result as string)
    }
    reader.readAsDataURL(selectedFile)
  }, [validateFile])

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    const files = e.dataTransfer.files
    if (files.length > 0) {
      handleFileSelect(files[0])
    }
  }, [handleFileSelect])

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault()
  }, [])

  const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (files && files.length > 0) {
      handleFileSelect(files[0])
    }
  }, [handleFileSelect])

  const handleAddWatermark = useCallback(async () => {
    if (!file) {
      setError('请先上传图片')
      return
    }
    if (!watermarkText.trim()) {
      setError('请输入水印文字')
      return
    }

    setProcessing(true)
    setError(null)

    try {
      const arrayBuffer = await file.arrayBuffer()
      const inputData = new Uint8Array(arrayBuffer)

      const result = await watermark(inputData, {
        text: watermarkText,
        fontSize: fontSize,
        position: 'center',
        rotation: rotation,
        color: watermarkColor,
        fontWeight: fontWeight,
        spacing: spacing,
      })

      const mimeType = getMimeType('original')
      const url = toDataUrl(result, mimeType)

      setResultUrl(url)
      onProcessed?.(url)
    } catch (err) {
      console.error('Watermark error:', err)
      setError(`添加水印失败: ${err instanceof Error ? err.message : String(err)}`)
    } finally {
      setProcessing(false)
    }
  }, [file, watermarkText, fontSize, fontWeight, rotation, watermarkColor, spacing, onProcessed])

  const handleDownload = useCallback(() => {
    if (!resultUrl) return
    const link = document.createElement('a')
    link.href = resultUrl
    link.download = `watermarked_${file?.name || 'image'}`
    link.click()
  }, [resultUrl, file])

  const handleRemove = useCallback(() => {
    setFile(null)
    setPreview(null)
    setResultUrl(null)
    setWatermarkText('')
    setError(null)
  }, [])

  return (
    <div className="space-y-6">
      {/* Upload Zone */}
      <div
        className="border-2 border-dashed border-gray-300 rounded-xl p-6 transition-colors duration-200 cursor-pointer hover:border-gray-400 hover:bg-gray-50"
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onClick={() => document.getElementById('watermark-input')?.click()}
      >
        <input
          id="watermark-input"
          type="file"
          accept=".png,.jpg,.jpeg,.webp,image/png,image/jpeg,image/webp"
          onChange={handleInputChange}
          className="hidden"
        />
        <div className="flex flex-col items-center text-center">
          <div className="w-12 h-12 bg-gray-100 rounded-full flex items-center justify-center mb-3">
            <svg className="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          {file ? (
            <p className="text-sm text-gray-600">{file.name}</p>
          ) : (
            <>
              <p className="text-sm font-medium text-gray-700 mb-1">拖拽图片到此处上传</p>
              <p className="text-xs text-gray-500">点击选择文件 · 支持 PNG、JPG、WebP，最大 10MB</p>
            </>
          )}
        </div>
      </div>

      {/* Preview */}
      {preview && (
        <div className="rounded-lg p-4">
          <img src={preview} alt="Preview" className="max-h-48 mx-auto object-contain" />
        </div>
      )}

      {/* Options - Always Visible */}
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            水印文字
          </label>
          <input
            type="text"
            value={watermarkText}
            onChange={(e) => setWatermarkText(e.target.value)}
            placeholder="输入水印文字"
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            aria-label="水印文字"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            水印颜色
          </label>
          <div className="flex items-center gap-3">
            <input
              type="color"
              value={watermarkColor}
              onChange={(e) => setWatermarkColor(e.target.value)}
              className="w-10 h-10 border border-gray-300 rounded-lg cursor-pointer"
              aria-label="水印颜色"
            />
            <input
              type="text"
              value={watermarkColor}
              onChange={(e) => setWatermarkColor(e.target.value)}
              placeholder="#808080"
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              aria-label="颜色值"
            />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              文字大小: {fontSize}px
            </label>
            <input
              type="range"
              min="12"
              max="120"
              value={fontSize}
              onChange={(e) => setFontSize(Number(e.target.value))}
              className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
              aria-label="文字大小"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              文字粗细
            </label>
            <select
              value={fontWeight}
              onChange={(e) => setFontWeight(e.target.value as 'normal' | 'bold')}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              aria-label="文字粗细"
            >
              <option value="normal">正常</option>
              <option value="bold">粗体</option>
            </select>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              文字角度: {rotation}°
            </label>
            <input
              type="range"
              min="-180"
              max="180"
              value={rotation}
              onChange={(e) => setRotation(Number(e.target.value))}
              className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
              aria-label="文字角度"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              文字间距: {spacing}px
            </label>
            <input
              type="range"
              min="0"
              max="100"
              value={spacing}
              onChange={(e) => setSpacing(Number(e.target.value))}
              className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
              aria-label="文字间距"
            />
          </div>
        </div>

        {error && (
          <p className="text-sm text-red-600 bg-red-50 px-3 py-2 rounded-lg">{error}</p>
        )}

        <div className="flex gap-3">
          <Button
            onClick={handleAddWatermark}
            disabled={processing || !file || !watermarkText.trim()}
            className="flex-1"
          >
            {processing ? '处理中...' : '添加水印'}
          </Button>
          {file && (
            <Button variant="outline" onClick={handleRemove}>
              移除
            </Button>
          )}
        </div>
      </div>

      {/* Result */}
      {resultUrl && (
        <div className="space-y-4">
          <div className="rounded-lg p-4 bg-green-50">
            <p className="text-sm text-green-700 text-center mb-3">水印添加完成!</p>
            <img src={resultUrl} alt="Result" className="max-h-48 mx-auto object-contain" />
          </div>
          <Button onClick={handleDownload} className="w-full">
            <Download className="w-4 h-4 mr-2" />
            下载图片
          </Button>
        </div>
      )}
    </div>
  )
}
