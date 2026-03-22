import type {
  OutputFormat,
  CompressOptions,
  WatermarkOptions,
  PortraitOptions,
} from '../types/image-wasm';

// WASM module initialization
let wasmInitialized = false;
let wasmModule: typeof import('../../public/wasm/image_wasm') | null = null;

// Dynamic import for WASM
async function getWasmModule() {
  if (!wasmModule) {
    wasmModule = await import('../../public/wasm/image_wasm');
  }
  return wasmModule;
}

export async function initWasm(): Promise<void> {
  if (!wasmInitialized) {
    const wasm = await getWasmModule();
    await wasm.default();
    wasmInitialized = true;
  }
}

/**
 * Compress an image with specified level and format
 */
export async function compress(
  inputData: Uint8Array,
  options: CompressOptions
): Promise<Uint8Array> {
  await initWasm();
  const wasm = await getWasmModule();
  const format = options.format === 'original' ? undefined : options.format;

  const result = wasm.compress_image(inputData, {
    level: options.level,
    format,
  });

  return new Uint8Array(result);
}

/**
 * Add watermark to an image
 */
export async function watermark(
  inputData: Uint8Array,
  options: WatermarkOptions
): Promise<Uint8Array> {
  await initWasm();
  const wasm = await getWasmModule();

  const result = wasm.add_watermark(inputData, {
    text: options.text,
    fontSize: options.fontSize ?? 24,
    position: options.position ?? 'center',
    rotation: options.rotation ?? 0,
    color: options.color ?? '#ffffff',
    fontWeight: options.fontWeight ?? 'normal',
    spacing: options.spacing ?? 100,
  });

  return new Uint8Array(result);
}

/**
 * Create portrait photo with specified background color and size
 */
export async function createPortrait(
  inputData: Uint8Array,
  options: PortraitOptions
): Promise<Uint8Array> {
  await initWasm();
  const wasm = await getWasmModule();

  const result = wasm.create_portrait_photo(inputData, {
    background: options.background,
    size: options.size,
  });

  return new Uint8Array(result);
}

/**
 * Convert Uint8Array to base64 data URL
 */
export function toDataUrl(data: Uint8Array, mimeType: string): string {
  let binary = '';
  const len = data.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(data[i]);
  }
  const base64 = btoa(binary);
  return `data:${mimeType};base64,${base64}`;
}

/**
 * Get MIME type from format
 */
export function getMimeType(format: OutputFormat): string {
  const mimeTypes: Record<string, string> = {
    jpeg: 'image/jpeg',
    jpg: 'image/jpeg',
    png: 'image/png',
    webp: 'image/webp',
    gif: 'image/gif',
    bmp: 'image/bmp',
    tiff: 'image/tiff',
    tif: 'image/tiff',
  };
  return mimeTypes[format] || 'image/png';
}

/**
 * Get file extension from format
 */
export function getExtension(format: OutputFormat): string {
  return format === 'jpeg' ? 'jpg' : format;
}

/**
 * Download data as file
 */
export function downloadData(data: Uint8Array, filename: string, mimeType: string): void {
  const dataUrl = toDataUrl(data, mimeType);
  const a = document.createElement('a');
  a.href = dataUrl;
  a.download = filename;
  a.click();
}
