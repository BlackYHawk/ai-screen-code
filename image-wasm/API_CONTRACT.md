# Image WASM API Contract

## Watermark API

### add_watermark(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue>

Adds a watermark to an image.

#### Input Parameters

- `input_data`: Image bytes (PNG, JPEG, WebP)
- `options`: WatermarkConfig object

#### WatermarkConfig

```typescript
interface WatermarkConfig {
  text: string;              // Watermark text content
  fontSize: number;           // Font size in pixels (default: 24)
  color: string;              // Hex color format "#RRGGBB" or "#RRGGBBAA" (default: "#000000")
  position: WatermarkPosition;  // Position enum
  rotation: number;           // Rotation angle in degrees 0-360 (default: 0)
  fontWeight: FontWeight;     // Font weight (default: "normal")
}
```

#### WatermarkPosition Enum

9 supported positions:
- `top-left` - Top left corner
- `top-center` - Top edge center
- `top-right` - Top right corner
- `middle-left` - Left edge center
- `center` - Image center
- `middle-right` - Right edge center
- `bottom-left` - Bottom left corner
- `bottom-center` - Bottom edge center
- `bottom-right` - Bottom right corner (default)

#### FontWeight Enum

- `normal` - Regular font weight (default)
- `bold` - Bold font weight

#### Output

Returns watermarked image as PNG bytes.

#### Example Usage (JavaScript/TypeScript)

```javascript
import init, { add_watermark } from 'image-wasm';

await init();

const imageData = await fetch('input.png').then(r => r.arrayBuffer());
const inputBytes = new Uint8Array(imageData);

const options = {
  text: "Copyright 2024",
  fontSize: 24,
  color: "#FF000080",  // Red with 50% opacity
  position: "bottom-right",
  rotation: 45,
  fontWeight: "bold"
};

const result = add_watermark(inputBytes, options);
// result is PNG bytes
```

## Compression API

### compress_image(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue>

Compresses an image with specified level and format.

#### CompressionConfig

```typescript
interface CompressionConfig {
  level: CompressionLevel;  // Compression level
  format: OutputFormat;      // Output format
}
```

#### CompressionLevel Enum

- `light` - Quality priority
- `normal` - Balanced (default)
- `strong` - Size priority
- `extreme` - Smallest size

#### OutputFormat Enum

- `jpeg` - JPEG format
- `png` - PNG format (default)
- `webp` - WebP format

## Portrait API

### create_portrait_photo(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue>

Creates a portrait photo with specified background and size.

#### PortraitConfig

```typescript
interface PortraitConfig {
  background: BackgroundColor;  // Background color
  size: PhotoSize;               // Photo size
}
```

#### BackgroundColor Enum

- `white` - White background (default)
- `red` - Red background (RGB: 220, 53, 69)
- `blue` - Blue background (RGB: 0, 123, 255)

#### PhotoSize Enum

Standard photo sizes (in mm at 300 DPI):
- `one-inch` - 25x35mm (295x413 pixels)
- `one-inch-small` - 22x32mm (260x378 pixels)
- `one-inch-large` - 33x48mm (390x567 pixels)
- `two-inch` - 35x49mm (413x579 pixels) (default)
- `two-inch-small` - 35x45mm (413x531 pixels)
- `two-inch-large` - 35x53mm (413x626 pixels)
- `three-inch` - 58x84mm (685x992 pixels)
- `four-inch` - 76x102mm (898x1205 pixels)
- `five-inch` - 89x127mm (1051x1500 pixels)

#### Example Usage (JavaScript/TypeScript)

```javascript
import init, { create_portrait_photo } from 'image-wasm';

await init();

const imageData = await fetch('portrait.jpg').then(r => r.arrayBuffer());
const inputBytes = new Uint8Array(imageData);

// Convert to white background, two-inch size
const options = {
  background: "white",
  size: "two-inch"
};

const result = create_portrait_photo(inputBytes, options);
// result is PNG bytes with white background

// Convert to red background, one-inch size
const redOptions = {
  background: "red",
  size: "one-inch"
};

const redResult = create_portrait_photo(inputBytes, redOptions);

// Convert to blue background, three-inch size
const blueOptions = {
  background: "blue",
  size: "three-inch"
};

const blueResult = create_portrait_photo(inputBytes, blueOptions);
```

#### DPI Standard

All photo sizes are calculated at 300 DPI (dots per inch), which is the standard for printing:

- 1 inch = 25.4mm
- 300 DPI = 300 pixels per inch
- pixels = mm × 300 / 25.4
