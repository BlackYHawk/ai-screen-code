import { useState, useCallback } from 'react'
import { Download } from 'lucide-react'
import { compress, toDataUrl, getMimeType } from '@/utils/imageProcessor'
import type { CompressionLevel, OutputFormat } from '@/types/image-wasm'
import { Button } from '@/components/common'

const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10MB

interface CompressToolProps {
  onProcessed?: (url: string) => void
}

export function CompressTool({ onProcessed }: CompressToolProps) {
  const [file, setFile] = useState<File | null>(null)
  const [preview, setPreview] = useState<string | null>(null)
  const [quality, setQuality] = useState(80)
  const [format, setFormat] = useState<OutputFormat>('original')
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

  const handleCompress = useCallback(async () => {
    if (!file) {
      setError('请先上传图片')
      return
    }

    setProcessing(true)
    setError(null)

    try {
      const arrayBuffer = await file.arrayBuffer()
      const inputData = new Uint8Array(arrayBuffer)

      const level: CompressionLevel = quality > 75 ? 'light' : quality > 50 ? 'normal' : quality > 25 ? 'strong' : 'extreme'

      const outputFormat = format === 'original' ? 'jpeg' : format
      const result = await compress(inputData, { level, format: outputFormat })
      const mimeType = getMimeType(outputFormat)
      const url = toDataUrl(result, mimeType)

      setResultUrl(url)
      onProcessed?.(url)
    } catch (_err) {
      setError('图片压缩失败，请重试')
    } finally {
      setProcessing(false)
    }
  }, [file, quality, format, onProcessed])

  const handleDownload = useCallback(() => {
    if (!resultUrl) return
    const ext = format === 'original' ? file?.name.split('.').pop() || 'jpg' : format === 'jpeg' ? 'jpg' : format
    const baseName = file?.name.replace(/\.[^.]+$/, '') || 'image'
    const link = document.createElement('a')
    link.href = resultUrl
    link.download = `compressed_${baseName}.${ext}`
    link.click()
  }, [resultUrl, file, format])

  const handleRemove = useCallback(() => {
    setFile(null)
    setPreview(null)
    setResultUrl(null)
    setError(null)
  }, [])

  return (
    <div className="space-y-6">
      {/* Upload Zone */}
      <div
        className="border-2 border-dashed border-gray-300 rounded-xl p-6 transition-colors duration-200 cursor-pointer hover:border-gray-400 hover:bg-gray-50"
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onClick={() => document.getElementById('compress-input')?.click()}
      >
        <input
          id="compress-input"
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
            压缩质量: {quality}%
          </label>
          <input
            type="range"
            min="10"
            max="100"
            value={quality}
            onChange={(e) => setQuality(Number(e.target.value))}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
            aria-label="压缩质量"
          />
          <div className="flex justify-between text-xs text-gray-400 mt-1">
            <span>低压缩</span>
            <span>高压缩</span>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            输出格式
          </label>
          <select
            value={format}
            onChange={(e) => setFormat(e.target.value as OutputFormat)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            aria-label="输出格式"
          >
            <option value="original">保持原格式</option>
            <option value="jpeg">JPG</option>
            <option value="png">PNG</option>
            <option value="webp">WebP</option>
          </select>
        </div>

        {error && (
          <p className="text-sm text-red-600 bg-red-50 px-3 py-2 rounded-lg">{error}</p>
        )}

        <div className="flex gap-3">
          <Button
            onClick={handleCompress}
            disabled={processing || !file}
            className="flex-1"
          >
            {processing ? '处理中...' : '压缩图片'}
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
            <p className="text-sm text-green-700 text-center mb-3">压缩完成!</p>
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
