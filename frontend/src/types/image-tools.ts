export type ToolType = 'compress' | 'watermark' | 'portrait'

export type ToolState = 'idle' | 'uploading' | 'processing' | 'done' | 'error'

export interface CompressState {
  file: File | null
  preview: string | null
  quality: number
  processing: boolean
  resultUrl: string | null
}

export interface WatermarkState {
  file: File | null
  preview: string | null
  watermarkText: string
  position: 'center' | 'tiled' | 'corner'
  opacity: number
  processing: boolean
  resultUrl: string | null
}

export interface PortraitState {
  file: File | null
  preview: string | null
  format: '1寸' | '2寸' | '签证'
  bgColor: 'white' | 'blue' | 'red'
  processing: boolean
  resultUrl: string | null
}
