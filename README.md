# EInk Image Converter

A Rust utility for converting images to formats optimized for eink displays. Based on research into monochrome e-paper display characteristics and rendering techniques.

## Features

- **Floyd-Steinberg dithering** (enabled by default) - Creates smooth gradients perfect for eink
- **Gamma correction** - Processes images in linear light space for accurate dithering
- **Tunable error diffusion** - Prevents "pepper noise" in highlights while preserving detail
- **Contrast enhancement** - Optimizes limited dynamic range of eink displays
- **Adjustable threshold** - Fine-tune black/white decision point
- **Multiple format support** - PNG, JPEG, BMP, and more

## Quick Start

```bash
# Basic usage with optimal eink defaults
cargo run -- -i photo.jpg -o eink_photo.png

# Fine-tune for specific images
cargo run -- -i portrait.jpg -o output.png --diffusion 0.75 --gamma 2.4 -c 1.2

# High contrast mode for graphics
cargo run -- -i logo.png -o output.png --threshold 100 --diffusion 0.6

# Disable dithering for text/simple graphics
cargo run -- -i text.jpg -o output.png --no-dither
```

## Command Line Options

| Option | Default | Description |
|--------|---------|-------------|
| `-i, --input` | - | Input image file (required) |
| `-o, --output` | - | Output image file (required) |
| `-c, --contrast` | 1.3 | Contrast enhancement level (0.0-2.0) |
| `-g, --gamma` | 2.2 | Gamma correction value |
| `-t, --threshold` | 128 | Dithering threshold (0-255) |
| `--diffusion` | 0.8 | Error diffusion amount (0.0-1.0) |
| `--no-dither` | false | Disable Floyd-Steinberg dithering |

## Understanding the Parameters

### Diffusion Amount (--diffusion)
Controls how much quantization error is spread to neighboring pixels:
- **0.8 (default)**: Optimal for most photos - prevents noise in highlights
- **0.6-0.7**: More contrasty, good for portraits
- **0.9-1.0**: Maximum detail preservation, may look noisy
- **0.0**: No error diffusion (equivalent to simple thresholding)

### Gamma Correction (--gamma)
Converts images to linear light space for accurate dithering:
- **2.2 (default)**: Standard sRGB gamma
- **2.4**: Alternative gamma for some displays
- **1.0**: No gamma correction (linear processing)

### Threshold (--threshold)
The brightness level that determines black vs white:
- **128 (default)**: Standard 50% threshold
- **100-120**: Darker images, more white pixels
- **140-160**: Brighter images, more black pixels

### Contrast Enhancement (--contrast)
Boosts contrast to utilize eink's limited dynamic range:
- **1.3 (default)**: Mild enhancement for most images
- **1.5-2.0**: Strong enhancement for flat images
- **1.0**: No contrast adjustment

## Technical Background

This utility implements research-based techniques for optimal eink rendering:

1. **Linear Light Processing**: Gamma correction ensures dithering operates in perceptually uniform space
2. **Tuned Error Diffusion**: Reduced diffusion (80% vs 100%) prevents "pepper noise" in light areas
3. **Dynamic Range Optimization**: Contrast enhancement and threshold tuning maximize eink's limited tonal range
4. **Integer Math**: All processing uses integer arithmetic for embedded compatibility

### Why These Defaults?

- **Dithering enabled**: Essential for photographic content on 1-bit displays
- **80% diffusion**: Balances detail preservation with clean highlights
- **Gamma 2.2**: Matches sRGB source images for accurate tone mapping
- **Contrast 1.3**: Mild boost to overcome eink's limited reflectance range

## Performance Notes

- **Memory efficient**: Streams processing to minimize RAM usage
- **Integer arithmetic**: No floating point operations in critical paths
- **Lookup tables**: Pre-computed gamma correction for speed
- **Embedded ready**: Compatible with no_std Rust environments

## Image Processing Pipeline

1. **Decode** - Load source image (JPEG, PNG, etc.)
2. **Grayscale** - Convert to single channel luminance
3. **Gamma correction** - Transform to linear light space
4. **Contrast enhancement** - Optimize dynamic range
5. **Dithering** - Apply Floyd-Steinberg with tuned diffusion
6. **Output** - Save optimized 1-bit image

## Use Cases

- **TRMNL displays**: Optimized for 7.5" monochrome e-paper
- **E-readers**: Custom firmware image processing
- **Embedded systems**: Low-resource image conversion
- **Art projects**: Newspaper/stipple aesthetic effects

## Building

```bash
cargo build --release
```

## Examples

### Photo Processing
```bash
# Portrait with clean highlights
cargo run -- -i portrait.jpg -o result.png --diffusion 0.75 -c 1.2

# Landscape with detail preservation
cargo run -- -i landscape.jpg -o result.png --diffusion 0.85 -c 1.4
```

### Graphics Processing
```bash
# Logo/text (no dithering)
cargo run -- -i logo.png -o result.png --no-dither -c 1.5

# Line art with mild dithering
cargo run -- -i drawing.png -o result.png --diffusion 0.5 --threshold 120
```

## Research Sources

Based on established practices in e-ink image processing, including:
- Error diffusion algorithms for limited palette displays
- Gamma correction for perceptually uniform dithering
- Dynamic range optimization for reflective displays
- Embedded system optimization techniques

The implementation prioritizes both visual quality and computational efficiency for resource-constrained environments.
