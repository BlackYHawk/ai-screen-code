export type CompressionLevel = 'light' | 'normal' | 'strong' | 'extreme';

export type OutputFormat = 'original' | 'jpeg' | 'png' | 'webp' | 'bmp' | 'gif' | 'tiff';

export interface CompressOptions {
  level: CompressionLevel;
  format?: OutputFormat;
}

export type WatermarkPosition =
  | 'top-left'
  | 'top-center'
  | 'top-right'
  | 'center-left'
  | 'center'
  | 'center-right'
  | 'bottom-left'
  | 'bottom-center'
  | 'bottom-right';

export interface WatermarkOptions {
  text: string;
  fontSize?: number;
  position?: WatermarkPosition;
  rotation?: number;
  color?: string;
  fontWeight?: 'normal' | 'bold';
  spacing?: number;
}

export type BackgroundColor = 'white' | 'red' | 'blue';

export type PhotoSize =
  | 'one-inch'
  | 'one-inch-small'
  | 'one-inch-large'
  | 'two-inch'
  | 'two-inch-small'
  | 'two-inch-large'
  | 'three-inch'
  | 'four-inch'
  | 'five-inch';

export interface PortraitOptions {
  background: BackgroundColor;
  size: PhotoSize;
}
