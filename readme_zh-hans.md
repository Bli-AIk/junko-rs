<div align="center">

# junko-rs

[![license](https://img.shields.io/badge/license-GPLv3-blue)](LICENSE.txt)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

<sub>*嫦娥啊。你在看着吗！？*</sub>

> 当前状态: 🚧 早期开发中 (Initial version in progress)

</div>

**junko-rs** — 使用 Rust 和 Bevy 重制的东方绀珠传六面。

| [English](./readme.md) | 简体中文 |
|------------------------|------|

## 简介

这是一个使用 Rust 编程语言和 Bevy 游戏引擎开发的东方绀珠传（Touhou LoLK / 東方紺珠伝 / Touhou
15）六面复刻项目。项目旨在重现原版游戏的玩法和视觉效果，同时利用 Bevy 引擎的强大功能来提升游戏性能和可扩展性。

这个项目最大的特点是使用了 **[GensokyoDSP](https://github.com/Bli-AIk/GensokyoDSP)** 项目，实现了**完全程序化的音频生成**
。这意味着整个项目**不包含任何版权音频文件**——所有音效和音乐均通过代码生成，并能够根据游戏状态动态调整音频效果。

## 特性

* 完整复刻东方绀珠传六面
* 使用 Rust 和 Bevy 游戏引擎构建
* 通过 GensokyoDSP 实现程序化音频生成
* 不含任何版权音频文件——所有音频由代码生成
* 音频可根据游戏状态动态调整
* （计划中）更多关卡和游戏模式

## 构建说明

### 前置要求

* Rust 1.70 或更高版本
* Bevy 依赖项（见下方）

### 构建步骤

1. **克隆仓库**：
   ```bash
   git clone https://github.com/Bli-AIk/junko-rs.git
   cd junko-rs
   ```

2. **构建并运行**：
   ```bash
   cargo run
   ```

3. **发布版本构建**：
   ```bash
   cargo build --release
   ```

## 依赖项

本项目使用以下主要依赖：

| Crate                                 | 版本     | 说明        |
|---------------------------------------|--------|-----------|
| [bevy](https://crates.io/crates/bevy) | 0.17.3 | 游戏引擎      |
| GensokyoDSP                           | -      | 程序化音频 DSP |

## 贡献

欢迎贡献！无论是修复错误、添加功能还是改进文档：

* 提交 **Issue** 或 **Pull Request**
* 分享想法，讨论设计或架构

## 许可证

本项目使用 GNU 通用公共许可证 v3.0（GPLv3）授权。

完整许可证文本请见 [LICENSE.txt](LICENSE.txt)。

### 东方 Project 使用规定

本项目遵守由 ZUN 和上海爱丽丝幻乐团制定的[东方Project使用规定案 / 东方同人规约](https://touhou-project.news/guideline/)
。作为同人作品，本项目：

* 非商业用途
* 不包含原作游戏的版权素材
* 适当标注原作信息
* 不冒充官方内容

---

<div align="center">
   <sub>
     <span style="
       color: #f958fb;
       text-shadow: 0 0 2px #000, 0 0 4px #000;
     ">
       Stage 6 静海（里）
     </span>
     <br>
     <b>
       <span style="
         font-size: 1.5em;
         color: #ffffff;
         text-shadow: 0 0 2px #000, 0 0 4px #000;
       ">
         即使已然不共戴天
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
<sub>原作 - 東方Project © 上海アリス幻樂団</sub>
</div>
