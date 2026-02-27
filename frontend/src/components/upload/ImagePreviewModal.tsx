import { useState, useCallback, useEffect } from 'react'
import { X, ZoomIn, ZoomOut, RotateCcw, ChevronLeft, ChevronRight, Download } from 'lucide-react'
import { Button } from '../common'

interface ImagePreviewModalProps {
  src: string
  alt?: string
  onClose: () => void
}

export function ImagePreviewModal({ src, alt = 'Preview', onClose }: ImagePreviewModalProps) {
  const [scale, setScale] = useState(1)
  const [position, setPosition] = useState({ x: 0, y: 0 })
  const [isDragging, setIsDragging] = useState(false)
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 })

  const MIN_SCALE = 0.5
  const MAX_SCALE = 4
  const SCALE_STEP = 0.25

  // Reset state when opening new image
  useEffect(() => {
    setScale(1)
    setPosition({ x: 0, y: 0 })
  }, [src])

  // Handle escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose()
      } else if (e.key === '+' || e.key === '=') {
        handleZoomIn()
      } else if (e.key === '-') {
        handleZoomOut()
      } else if (e.key === '0') {
        handleReset()
      } else if (e.key === 'ArrowLeft') {
        handleRotate(-90)
      } else if (e.key === 'ArrowRight') {
        handleRotate(90)
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [onClose])

  const handleZoomIn = useCallback(() => {
    setScale((prev) => Math.min(prev + SCALE_STEP, MAX_SCALE))
  }, [])

  const handleZoomOut = useCallback(() => {
    setScale((prev) => Math.max(prev - SCALE_STEP, MIN_SCALE))
  }, [])

  const handleReset = useCallback(() => {
    setScale(1)
    setPosition({ x: 0, y: 0 })
  }, [])

  const handleRotate = useCallback((degrees: number) => {
    // Rotate is visual only, we'll use CSS transform
    const img = document.getElementById('preview-image')
    if (img) {
      const currentRotation = img.style.transform.match(/rotate\((\d+)deg\)/)
      const currentDeg = currentRotation ? parseInt(currentRotation[1]) : 0
      const newDeg = currentDeg + degrees
      img.style.transform = `scale(${scale}) rotate(${newDeg}deg)`
    }
  }, [scale])

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    if (scale > 1) {
      setIsDragging(true)
      setDragStart({ x: e.clientX - position.x, y: e.clientY - position.y })
    }
  }, [scale, position])

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (isDragging) {
        setPosition({
          x: e.clientX - dragStart.x,
          y: e.clientY - dragStart.y,
        })
      }
    },
    [isDragging, dragStart]
  )

  const handleMouseUp = useCallback(() => {
    setIsDragging(false)
  }, [])

  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault()
    if (e.deltaY < 0) {
      handleZoomIn()
    } else {
      handleZoomOut()
    }
  }, [handleZoomIn, handleZoomOut])

  const handleDownload = useCallback(() => {
    const link = document.createElement('a')
    link.href = src
    link.download = `screenshot-${Date.now()}.png`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }, [src])

  const handleBackdropClick = useCallback((e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose()
    }
  }, [onClose])

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm"
      onClick={handleBackdropClick}
    >
      {/* Controls */}
      <div className="absolute top-4 left-1/2 -translate-x-1/2 flex items-center gap-2 bg-gray-900/90 px-4 py-2 rounded-full shadow-lg z-10">
        <Button
          variant="ghost"
          size="sm"
          onClick={handleZoomOut}
          disabled={scale <= MIN_SCALE}
          className="text-white hover:bg-white/20"
          title="缩小 (键盘: -)"
        >
          <ZoomOut className="w-4 h-4" />
        </Button>
        <span className="text-white text-sm min-w-[60px] text-center">
          {Math.round(scale * 100)}%
        </span>
        <Button
          variant="ghost"
          size="sm"
          onClick={handleZoomIn}
          disabled={scale >= MAX_SCALE}
          className="text-white hover:bg-white/20"
          title="放大 (键盘: +)"
        >
          <ZoomIn className="w-4 h-4" />
        </Button>
        <div className="w-px h-6 bg-white/30 mx-1" />
        <Button
          variant="ghost"
          size="sm"
          onClick={handleReset}
          className="text-white hover:bg-white/20"
          title="重置 (键盘: 0)"
        >
          <RotateCcw className="w-4 h-4" />
        </Button>
        <div className="w-px h-6 bg-white/30 mx-1" />
        <Button
          variant="ghost"
          size="sm"
          onClick={() => handleRotate(-90)}
          className="text-white hover:bg-white/20"
          title="左转 (键盘: ←)"
        >
          <ChevronLeft className="w-4 h-4" />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => handleRotate(90)}
          className="text-white hover:bg-white/20"
          title="右转 (键盘: →)"
        >
          <ChevronRight className="w-4 h-4" />
        </Button>
        <div className="w-px h-6 bg-white/30 mx-1" />
        <Button
          variant="ghost"
          size="sm"
          onClick={handleDownload}
          className="text-white hover:bg-white/20"
          title="下载图片"
        >
          <Download className="w-4 h-4" />
        </Button>
      </div>

      {/* Close button */}
      <button
        onClick={onClose}
        className="absolute top-4 right-4 p-2 bg-gray-900/90 text-white rounded-full hover:bg-white/20 transition-colors z-10"
        title="关闭 (Esc)"
      >
        <X className="w-6 h-6" />
      </button>

      {/* Image container */}
      <div
        className="relative overflow-hidden"
        style={{
          maxWidth: '90vw',
          maxHeight: '85vh',
        }}
      >
        <img
          id="preview-image"
          src={src}
          alt={alt}
          className="max-w-full max-h-[85vh] object-contain select-none transition-transform duration-200"
          style={{
            transform: `scale(${scale}) translate(${position.x / scale}px, ${position.y / scale}px)`,
            cursor: scale > 1 ? (isDragging ? 'grabbing' : 'grab') : 'default',
          }}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onWheel={handleWheel}
          draggable={false}
        />
      </div>

      {/* Hint */}
      <div className="absolute bottom-4 left-1/2 -translate-x-1/2 text-white/60 text-xs bg-black/50 px-3 py-1.5 rounded-full">
        使用滚轮缩放 | 拖拽移动 | Esc 关闭
      </div>
    </div>
  )
}

// Thumbnail component for gallery view
interface ImageThumbnailProps {
  src: string
  alt?: string
  selected?: boolean
  onClick?: () => void
  onRemove?: () => void
}

export function ImageThumbnail({ src, alt, selected, onClick, onRemove }: ImageThumbnailProps) {
  return (
    <div
      className={`
        relative group rounded-lg overflow-hidden cursor-pointer
        border-2 transition-all duration-200
        ${selected ? 'border-blue-500 ring-2 ring-blue-500/50' : 'border-transparent hover:border-gray-300'}
      `}
      onClick={onClick}
    >
      <img
        src={src}
        alt={alt}
        className="w-full h-20 object-cover"
      />
      {onRemove && (
        <button
          onClick={(e) => {
            e.stopPropagation()
            onRemove()
          }}
          className="absolute top-1 right-1 p-1 bg-red-500 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <X className="w-3 h-3" />
        </button>
      )}
    </div>
  )
}
