import { useState, useCallback, useRef, useEffect } from 'react'
import { Upload, X, Image as ImageIcon, ZoomIn } from 'lucide-react'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { Button } from '../common'
import { ImagePreviewModal } from './ImagePreviewModal'
import { isTauri } from '@/utils/runtime'

interface ImageUploadProps {
  onUpload: (file: File) => void
  currentImage?: string
  onRemove?: () => void
  uploadProgress?: number
  isUploading?: boolean
}

const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10MB
const ACCEPTED_TYPES = {
  'image/png': ['.png'],
  'image/jpeg': ['.jpg', '.jpeg'],
  'image/webp': ['.webp'],
}

export function ImageUpload({
  onUpload,
  currentImage,
  onRemove,
  uploadProgress,
  isUploading,
}: ImageUploadProps) {
  const [isDragActive, setIsDragActive] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showPreview, setShowPreview] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)

  const validateFile = useCallback((file: File): string | null => {
    if (!ACCEPTED_TYPES[file.type as keyof typeof ACCEPTED_TYPES]) {
      return '不支持的图片格式，请上传 PNG、JPG、JPEG 或 WebP 格式'
    }
    if (file.size > MAX_FILE_SIZE) {
      return '图片大小不能超过 10MB'
    }
    return null
  }, [])

  const handleFile = useCallback(
    (file: File) => {
      const validationError = validateFile(file)
      if (validationError) {
        setError(validationError)
        return
      }
      setError(null)
      onUpload(file)
    },
    [onUpload, validateFile]
  )

  const handleDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault()
      e.stopPropagation()
      setIsDragActive(false)

      if (currentImage || isUploading) return

      const files = e.dataTransfer.files
      if (files.length > 0) {
        handleFile(files[0])
      }
    },
    [currentImage, isUploading, handleFile]
  )

  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault()
    e.stopPropagation()
  }, [])

  const handleDragEnter = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault()
    e.stopPropagation()
    setIsDragActive(true)
  }, [])

  const handleDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault()
    e.stopPropagation()
    setIsDragActive(false)
  }, [])

  const handleClick = useCallback(async () => {
    if (!currentImage && !isUploading) {
      // Use native file dialog in Tauri
      if (isTauri()) {
        try {
          const selected = await open({
            multiple: false,
            filters: [
              {
                name: 'Image',
                extensions: ['png', 'jpg', 'jpeg', 'webp'],
              },
            ],
          })
          if (selected) {
            const filePath = selected as string
            const fileData = await readFile(filePath)
            const blob = new Blob([fileData])
            const ext = filePath.split('.').pop()?.toLowerCase() || 'png'
            const mimeType = ext === 'png' ? 'image/png' : ext === 'webp' ? 'image/webp' : 'image/jpeg'
            const file = new File([blob], `screenshot.${ext}`, { type: mimeType })
            handleFile(file)
          }
        } catch (err) {
          console.error('Failed to open file dialog:', err)
          // Fall back to web input
          inputRef.current?.click()
        }
      } else {
        inputRef.current?.click()
      }
    }
  }, [currentImage, isUploading, handleFile])

  const handleInputChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files
      if (files && files.length > 0) {
        handleFile(files[0])
      }
      // Reset input value to allow selecting same file again
      e.target.value = ''
    },
    [handleFile]
  )

  const handleRemove = useCallback(
    (e: React.MouseEvent<HTMLButtonElement>) => {
      e.stopPropagation()
      setError(null)
      onRemove?.()
    },
    [onRemove]
  )

  // Reset error when currentImage changes
  useEffect(() => {
    setError(null)
  }, [currentImage])

  if (currentImage) {
    return (
      <>
        <div className="relative group">
          <div
            className="relative cursor-pointer rounded-lg overflow-hidden border border-gray-200 bg-gray-50"
            onClick={() => setShowPreview(true)}
          >
            <img
              src={currentImage}
              alt="Uploaded"
              className="w-full h-64 object-contain"
            />
            {/* Hover overlay */}
            <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-all duration-200 flex items-center justify-center">
              <div className="opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                <Button variant="secondary" size="sm">
                  <ZoomIn className="w-4 h-4 mr-1" />
                  预览
                </Button>
              </div>
            </div>
          </div>
          {onRemove && (
            <button
              onClick={handleRemove}
              className="absolute top-2 right-2 p-1.5 bg-red-500/90 text-white rounded-full hover:bg-red-600 transition-colors shadow-md"
              title="移除图片"
            >
              <X className="w-4 h-4" />
            </button>
          )}
        </div>

        {showPreview && currentImage && (
          <ImagePreviewModal
            src={currentImage}
            onClose={() => setShowPreview(false)}
          />
        )}
      </>
    )
  }

  if (isUploading) {
    return (
      <div className="border-2 border-dashed border-gray-200 rounded-xl p-8">
        <div className="flex flex-col items-center text-center">
          <div className="w-16 h-16 rounded-full flex items-center justify-center mb-4 bg-blue-50">
            <Upload className="w-8 h-8 text-blue-600 animate-pulse" />
          </div>
          <p className="text-lg font-medium text-gray-700 mb-1">上传中...</p>
          {uploadProgress !== undefined && (
            <div className="w-full max-w-xs mt-4">
              <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
                <div
                  className="h-full bg-blue-600 rounded-full transition-all duration-300"
                  style={{ width: `${uploadProgress}%` }}
                />
              </div>
              <p className="text-sm text-gray-500 mt-2">{uploadProgress}%</p>
            </div>
          )}
        </div>
      </div>
    )
  }

  return (
    <div
      className={`
        border-2 border-dashed rounded-xl p-8
        transition-colors duration-200 cursor-pointer
        ${
          isDragActive
            ? 'border-blue-500 bg-blue-50'
            : 'border-gray-300 hover:border-gray-400 hover:bg-gray-50'
        }
        ${error ? 'border-red-300 bg-red-50' : ''}
      `}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragEnter={handleDragEnter}
      onDragLeave={handleDragLeave}
      onClick={handleClick}
    >
      <input
        ref={inputRef}
        type="file"
        accept=".png,.jpg,.jpeg,.webp"
        onChange={handleInputChange}
        className="hidden"
      />
      <div className="flex flex-col items-center text-center">
        <div
          className={`
            w-16 h-16 rounded-full flex items-center justify-center mb-4
            ${isDragActive ? 'bg-blue-100' : 'bg-gray-100'}
          `}
        >
          {isDragActive ? (
            <ImageIcon className="w-8 h-8 text-blue-600" />
          ) : (
            <Upload className="w-8 h-8 text-gray-400" />
          )}
        </div>
        <p className="text-lg font-medium text-gray-700 mb-1">
          {isDragActive ? '松开以上传图片' : '拖拽图片到此处上传'}
        </p>
        <p className="text-sm text-gray-500 mb-4">或点击选择文件</p>
        <p className="text-xs text-gray-400">
          支持 PNG、JPG、JPEG、WebP 格式，最大 10MB
        </p>
        {error && (
          <p className="text-sm text-red-600 mt-4 bg-red-100 px-3 py-2 rounded-lg">
            {error}
          </p>
        )}
      </div>
    </div>
  )
}

interface ImageUploadWithPreviewProps {
  onUpload: (file: File) => void
  currentFile?: { preview: string } | null
  onRemove?: () => void
  uploadProgress?: number
  isUploading?: boolean
}

export function ImageUploadWithPreview({
  onUpload,
  currentFile,
  onRemove,
  uploadProgress,
  isUploading,
}: ImageUploadWithPreviewProps) {
  return (
    <ImageUpload
      onUpload={onUpload}
      currentImage={currentFile?.preview}
      onRemove={onRemove}
      uploadProgress={uploadProgress}
      isUploading={isUploading}
    />
  )
}
