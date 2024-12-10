# Quilter

Quilter is a Tauri app to help create Quilts for Looking Glass products. This app is designed to be used with the Blender plugin - more specifically with [our fork](https://github.com/transcental/AmberLG) of it. We have a currently [open PR](https://github.com/regcs/AliceLG/pull/139) to merge our changes into the original plugin.

## Features

- Sorting files
- Creating quilts
- Combining quilts into a video
- Auto updates

## Installation

Download the latest release from the [releases page](https://quilter-releases.transcental.dev/) - _If on an  Apple Silicon Mac, please select `darwin_arm64` and move the `.app` into your Applications folder_

## Development

### Prerequisites

- [Rust](https://rustup.rs)
- [Bun](https://bun.sh)

### Setup

1. Clone the repository
2. Install dependencies with `bun i`
3. Start the development server with `bun tauri dev`
