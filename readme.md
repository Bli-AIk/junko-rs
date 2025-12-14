<div align="center">

# junko-rs

[![license](https://img.shields.io/badge/license-GPLv3-blue)](LICENSE.txt)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

<sub>*Chang'e. Are you watching!?*</sub>

</div>

**junko-rs** — A Touhou Legacy of Lunatic Kingdom Stage 6 recreation using Rust and Bevy.

| English | [简体中文](./readme_zh-hans.md) |
|---------|----------------------------------|

## Introduction

This project is a recreation of Touhou Legacy of Lunatic Kingdom (Touhou LoLK / 東方紺珠伝 / Touhou 15) Stage 6, developed using the Rust programming language and the Bevy game engine. The goal is to recreate the original gameplay and visual effects while leveraging Bevy's powerful features to enhance performance and extensibility.

The most unique feature of this project is the use of **[GensokyoDSP](https://github.com/Bli-AIk/GensokyoDSP)**, which enables **fully procedural audio generation**. This means the project contains **no copyrighted audio files** — all sound effects and music are generated programmatically and can dynamically adapt to game states.

## Features

* Full recreation of Touhou LOLK Stage 6
* Built with Rust and Bevy game engine
* Procedurally generated audio via GensokyoDSP
* No copyrighted audio files — all audio is code-generated
* Dynamic audio that responds to gameplay
* (Planned) Additional stages and game modes

## How to Build

### Prerequisites

* Rust 1.70 or later
* Bevy dependencies (see below)

### Build Steps

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Bli-AIk/junko-rs.git
   cd junko-rs
   ```

2. **Build and run**:
   ```bash
   cargo run
   ```

3. **Build for release**:
   ```bash
   cargo build --release
   ```

## Dependencies

This project uses the following main dependencies:

| Crate                                      | Version | Description          |
|--------------------------------------------|---------|----------------------|
| [bevy](https://crates.io/crates/bevy)      | 0.17.3  | Game engine          |
| GensokyoDSP                                | -       | Procedural audio DSP |

## Contributing

Contributions are welcome! Whether you want to fix a bug, add a feature, or improve documentation:

* Submit an **Issue** or **Pull Request**
* Share ideas and discuss design or architecture

## License

This project is licensed under the GNU General Public License v3.0 (GPLv3).

See [LICENSE.txt](LICENSE.txt) for full license text.

### Touhou Project Guidelines

This project respects the [Touhou Project Usage Guidelines (東方Project使用規定案 / 東方同人規約)](https://touhou-project.news/guideline/)established by ZUN and Team Shanghai Alice. As a fan work, this project:

* Is non-commercial
* Contains no copyrighted assets from the original games
* Properly credits the original work
* Does not misrepresent itself as official content

---

<div align="center">
   <sub>
     <span style="
       color: #f958fb;
       text-shadow: 0 0 2px #000, 0 0 4px #000;
     ">
       Stage 6 Sea of Tranquility (Underside)
     </span>
     <br>
     <b>
       <span style="
         font-size: 1.5em;
         color: #ffffff;
         text-shadow: 0 0 2px #000, 0 0 4px #000;
       ">
         Though Unable to Dwell Together Under Heaven
       </span>
     </b>
     <br>
     <span style="
         font-size: 1.25em;
         color: #ffffff;
         text-shadow: 0 0 2px #000, 0 0 4px #000;
      ">
      Pure Furies
      </span>
   </sub>
</div>
<br>
<div align="center">
<sub>ORIGINAL - 東方Project © 上海アリス幻樂団</sub>
</div>
