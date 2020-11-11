# Boenthoe

Multiplatform Rust demo engine.

Everything is *still* very unfinished but it starts to come together.

Features:

- glTF support
- Multiple lightning models:
    - [Physically based rendering](https://en.wikipedia.org/wiki/Physically_based_rendering) (roughness-metallic)
    - Phong shading with and without normal maps
- Multi-platform support: Both **Windows** and **macOS**
- Own script language (BoenthoeScript a.k.a. bäsä) for simple scripting
- Hot-reload for script and assets (temporarily disabled).

Future steps:

- Ambient occlusion for PBR
- glTF animation support
- Particles
- Standard library of post-processing effects
- Linux support
- Killer demos

Maybe:

- Web support

## Setup

- [Rust and Cargo](https://rustup.rs/)
- On Windows: [Visual Studio 2019](https://visualstudio.microsoft.com/vs/)
- Nightly channel for experimental Rust feature `vec_into_raw_parts`: `rustup toolchain install nightly`
- Shaderc library or required build tools. See https://github.com/google/shaderc-rs

## License

Copyright 2020 Ilkka Hänninen

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
