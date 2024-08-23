<p align="center"><img src="./assets/basil-k.jpg" width=240></img></p>
<p align="center"><i>illustration generated using <a href="https://perchance.org/ai-pixel-art-generator">perchance.org</a></i></p>

<h1 align="center">basilk</h1>
<p align="center">A Terminal User Interface (TUI) to manage your tasks with minimal kanban logic</p>

<img src="./assets/basilk.gif"></img>

## History
It was a [very hot August night](https://www.meteo.it/notizie/meteo-caldo-in-aumento-la-tendenza-verso-ferragosto-c95aa7dc), and I was organizing my activities when at a certain point I felt the need for a software that could help me with this, something simple and portable. **basilk** is created as a summer project to learn Rust and to be able to use the software anywhere. 

The name [_/ˈbæzəlkeɪ/_](https://gabalpha.github.io/read-audio/?p=https://github.com/GabAlpha/basilk/raw/master/assets/basil-k.wav) comes from the basil plant, which is easy to grow and maintain, and "k" stands for kanban.

<details>
<summary>Another story</summary>

<p align="center"><img src="./assets/bas-silk.jpg" width=240></img></p>
<p align="center"><i>illustration generated using <a href="https://perchance.org/ai-pixel-art-generator">perchance.org</a></i></p>

The name [_/ˈbæzsɪlk/_](https://gabalpha.github.io/read-audio/?p=https://github.com/GabAlpha/basilk/raw/master/assets/bas-silk.wav) comes from the union of basil and silk as a symbol of elaborateness due to its production process.
</details>

## About
**basilk** is structured to create projects and within each project to create tasks with a specific status (Up Next/On Going/Done).

The data structure is saved in `.json` format and is available in the directory:
```
Linux
~/.config/basilk

macOS
~/Library/Application Support/basilk

Windows
<USER>\AppData\Roaming\basilk
```
The choice to use the JSON format is to make easier to export

## Installation
### Cargo

from [crates.io](https://crates.io/crates/basilk) using [`cargo`](https://doc.rust-lang.org/cargo/)

```sh
cargo install basilk
```

### AUR

from the [AUR](https://aur.archlinux.org/packages/basilk) with using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers).

```sh
paru -S basilk
```

### Build from source

1. Clone the repository
```sh
git clone https://github.com/GabAlpha/basilk && cd basilk
```
2. Build
```sh
cargo build --release
```
Binary will be located at `target/release/basilk`

## Usage
Run

```sh
basilk
```
All available commands are displayed inside

## Contributing
> [!NOTE]  
> This project is now in beta version and is expected to have bugs

As I mentioned above, this is my first project in Rust, so contributions and help are welcome! If you have any suggestions, improvements, or bug fixes, feel free to submit a pull request or open a new issue.

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.
