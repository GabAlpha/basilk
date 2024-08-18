<p align="center"><img src="./assets/illustration.jpg" width=240></img></p>
<p align="center"><i>illustration generated using <a href="https://perchance.org/ai-pixel-art-generator">perchance.org</a></i></p>

<h1 align="center">basil</h1>
<p align="center">A Terminal User Interface (TUI) to manage your tasks with the simplest kanban logic</p>

<img src="./assets/basil.gif"></img>

## History
It was a [very hot August night](https://www.meteo.it/notizie/meteo-caldo-in-aumento-la-tendenza-verso-ferragosto-c95aa7dc), and I was organizing my activities when at a certain point I felt the need for a software that could help me with this, something simple and portable. **basil** is created as a summer project to learn Rust and to be able to use the software anywhere. The name comes from the basil plant, [easy and simple](https://www.rhs.org.uk/herbs/basil/grow-your-own) to grow and maintain.

## About
**basil** is structured to create projects and within each project, to create activities with a specific status.

The database is saved in `.json` format and is available in the directory:
```
~/.config/basil
```
The choice to use the json format is to make the structure easier to export.

## Installation

### Build from source

1. Clone the repository
```sh
git clone https://github.com/GabAlpha/basil && cd basil
```
2. Build
```sh
cargo build --release
```
Binary will be located at `target/release/basil`

## Usage
Run

```sh
basil
```
All available commands are displayed inside

## Contributing
As I mentioned above, this is my first project in Rust, so contributions and help are welcome! If you have any suggestions, improvements, or bug fixes, feel free to submit a pull request or open a new issue.

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.
