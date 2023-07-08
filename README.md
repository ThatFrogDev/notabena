# Notabena
![CI checks](https://github.com/thatfrogdev/notabena/actions/workflows/ci-checks.yml/badge.svg) ![crates.io](https://img.shields.io/crates/v/notabena.svg)

## About
Notabena is the free and open source note-taking app, written in pure Rust.

## Features
These are our current planned features. (Most features under CLI are also featured in the GUI when it's released, for the two are planned to be fully compatible)

- [x] CLI
  - [x] Make notes
  - [x] Edit notes
  - [x] Delete notes
    - [x] Delete multiple notes at once
  - [x] Local SQL database
  - [ ] Encrypted notes
  - [ ] Simple Markdown support
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

## Linux Installation Notes
When running Notabena on Linux, you might encounter an error like this:<br>
`bash: /home/Your-Username/Downloads/Your-Notabena-Installation: Permission denied`<br>
To fix this issue, run the following command:<br>
`chmod u+x /home/Your-Username/Downloads/Your-Notabena-Installation` (filling in the blanks)<br>
The program should now run smoothly!
