![Banner](/static/banner.png)

![CI checks](https://github.com/thatfrogdev/notabena/actions/workflows/ci-checks.yml/badge.svg) ![crates.io](https://img.shields.io/crates/v/notabena.svg)

## About
Notabena is a free and open source note-taking app, written in pure Rust. [Join our Discord here!](https://discord.gg/htNK4YcJB8)

## Features
These are our current planned features. (Most features under CLI are also featured in the GUI when it's released, for the two are planned to be fully compatible)

- [x] CLI
  - [x] Make notes
  - [x] Edit notes
  - [x] Delete notes
    - [x] Delete multiple notes at once
  - [x] Local SQL database
  - [ ] Encrypted notes
  - [x] Simple Markdown support
  - [ ] Compatibility with the GUI
- [ ] GUI
  - [ ] Custom themes
  - [ ] Private vault: only accessible with PIN
  - [ ] Folders

## Help us!
You can help us in different ways.<br>
🐛 ・ Be a Bughunter: Search for bugs and file an issue if the output isn't as expected or if the program crashes.<br>
🖥️ ・ Suggest features: File an issue (use the Suggest features issue template) and the devs will look into it.<br>
🧹 ・ Fix issues: Are you a Rust developer and interested in the project: try to fix the issues and open a pull request! (Especially the ones tagged with `good first issue`)

**Contributing guidelines** for these three roles coming soon!

## Installation
There are a few different ways of installing Notabena:
- **Recommended way:** through SourceForge: https://sourceforge.net/projects/notabena/. This requires no knowledge or programs, but if you want to run it with `notabena` you'll need to add it to `PATH` manually. Is your architecture not in there? Consider building for all the versions and [sending the binaries in our server](https://discord.gg/htNK4YcJB8).
- **Package manager:** currently, Notabena only supports *Homebrew* for **MacOS** and **Linux**. The formula is in `chiissu/macchiato`. More package managers will be added at stable.
  - (Still unsure? `brew tap chiissu/macchiato && brew install notabena`)
- **The Rust way:** if you have Rust installed, we recommend installing it through `cargo` (`cargo install notabena`). It will automatically be added to your `PATH`.

When running Notabena on **Linux or macOS**, you might encounter an error like this when running the file:<br>
`bash: /home/Your-Username/Downloads/Your-Notabena-Installation: Permission denied`<br>
To fix this issue, **run the following command in your terminal:**<br>
Linux:
`chmod u+x /home/Your-Username/Downloads/Your-Notabena-Installation` (filling in the blanks)<br>
MacOS:
`chmod u+x /Users/Your-Username/Downloads/Your-Notabena-Installation` (filling in the blanks)<br>
The program should now run smoothly!<br>

There are no issues known with Windows installation. If you get another error or similar bug, please open an issue.
