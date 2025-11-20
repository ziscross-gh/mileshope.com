# Design System

## Overview
MilesHope.com uses a custom design system built on top of Tailwind CSS v4. The system is designed to be spiritual, modern, and accessible.

## Color Palette

### Primary Colors
- **Purple** (Primary Brand): `#805ad5` (`--color-purple`)
- **Gold** (Accent): `#d69e2e` (`--color-gold`)

### Dark Mode Variants
- **Purple Light**: `#b794f4` (`--color-purple-light`)
- **Gold Light**: `#fbd38d` (`--color-gold-light`)

### Neutrals
- **Gray 800**: `#1a202c` (Dark background)
- **Gray 100**: `#f7fafc` (Light background)

## Typography

### Font Families
- **Headings**: `Lora` (Serif) - Elegant, spiritual feel.
- **Body**: `Inter` (Sans-serif) - Clean, readable.

### Fluid Scale
Typography scales fluidly between mobile and desktop using `clamp()`.
- **H1**: 2rem - 3rem
- **H2**: 1.5rem - 2.25rem
- **Body**: 1rem - 1.125rem

## Components

### Buttons
- `.btn-primary`: Solid purple (light) / Gold (dark).
- `.btn-secondary`: Outline style.

### Cards
- `.post-card`: White background with subtle shadow. Dark mode: Gray 800.
- Hover effect: Lift up + shadow increase + border color change.

### Gradients
- `.hero-gradient`: Purple to Dark Purple.
- `.reading-progress`: Purple to Gold.

## Usage
All styles are defined in `styles/` and compiled via Tailwind CSS.
- `theme.css`: Variables & Configuration.
- `base.css`: Core element styles.
- `components/`: Reusable UI patterns.
