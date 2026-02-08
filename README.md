# rust.fractals

Experimenting with Rust and fractals. Because fractals are cool.

## Fractals

- **Mandelbrot** — the classic z = z^2 + c
- **Julia** — fixed c = -0.8 + 0.156i
- **Burning Ship** — abs of real/imaginary before squaring
- **Tricorn** — complex conjugate before squaring
- **Newton** — Newton's method on z^3 - 1, colored by root convergence

## Prerequisites

- [Rust](https://rustup.rs/) (1.85+ for rug crate)
- SDL2: `brew install sdl2 sdl2_ttf`
- GMP/MPFR (for arbitrary precision deep zoom): `brew install gmp mpfr`

## Build & Run

```bash
# Load Rust toolchain (or add to your ~/.zshrc permanently)
source "$HOME/.cargo/env"

# Build only
LIBRARY_PATH=/opt/homebrew/lib cargo build --release

# Build and run
LIBRARY_PATH=/opt/homebrew/lib cargo run --release

# Debug build (faster compile, slower runtime)
LIBRARY_PATH=/opt/homebrew/lib cargo run
```

## Controls

| Key | Action |
|---|---|
| 1-5 | Switch fractal |
| W / Up | Pan up |
| S / Down | Pan down |
| A / Left | Pan left |
| D / Right | Pan right |
| E / = | Zoom in |
| Q / - | Zoom out |
| Tab | Toggle menu overlay |
| Space | Toggle auto-play |
| R | Reset view |
| Escape | Quit |

## Auto-Play

Auto-play is **on by default**. It smoothly zooms into interesting points for each fractal and automatically cycles through all 5 fractal types. Press **Space** to toggle. For Mandelbrot, auto-play includes ultra-deep zoom locations (10^14 to 10^20) powered by perturbation theory.

## Performance Features

- **Pre-computed color palette** — 4096-entry LUT eliminates trig from the hot loop
- **Distance estimation** — derivative tracking skips fully-exterior Mandelbrot pixels
- **Cardioid/bulb skip** — instant interior detection for ~30% of Mandelbrot viewport
- **Periodicity checking** — exponential-backoff cycle detection for interior points
- **Perturbation theory** — arbitrary-precision reference orbit + f64 delta iteration kicks in automatically at zoom > 10^13 for Mandelbrot, enabling zoom to 10^50+
- **Rayon parallelism** — per-row parallel computation across all CPU cores
