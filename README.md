# ASCII Art Generator

A Rust program that converts images into colored ASCII art, optimized for terminal display.

## Features

- Converts any image to ASCII art with color preservation
- Adjusts image contrast for better output
- Handles aspect ratio correction for terminal characters
- Supports custom character sets and color palettes
- Resizes images while maintaining proportions

## Requirements

- Rust 1.70+ (edition 2021)
- Cargo package manager
- `libpng` development libraries (for image processing)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ascii-art-generator.git
   cd ascii-art-generator
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Place your input image in the project root as `input.png`
2. Run the program:
   ```bash
   cargo run --release
   ```

## Configuration

Modify these parameters in `main.rs`:
- `symbol_ratio` - Character width/height ratio (default 0.5 for most terminals)
- `target_height` - Output height in characters (default 100)
- `ascii_chars` - Character gradient from light to dark
- `colors` - Color palette for output
