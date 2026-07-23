# ISO to GOD Converter

A simple app coded using Claude that adds a GUI to the existing conversion tool for game ISOs into the Games-on-Demand format your Xbox can read straight off a hard drive. Pick your files, pick a folder, click Convert.

Works on macOS, Windows(untested), and Linux(untested).

## Download

**[⬇ Get the latest version here](../../releases/latest)**

Pick the file for your OS from the release's "Assets" list.

## Opening it the first time

Since this is a free, independent app (not sold through an app store), your computer will ask "are you sure?" the first time you open it. This is normal and only happens once.

- **macOS:** Right-click the app → **Open** → **Open** again in the popup.
- **Windows:** Click **More info** → **Run anyway** on the blue popup.
- **Linux:** No warning — just make it executable and run it.

## How to use it

1. First launch, it asks how many threads to use, how to trim the ISO, and whether to ask again next time or just remember your choices.
2. Pick your ISO file(s) and a folder to save the converted files to.
3. Click **Convert** and wait for it to finish.
4. Want to change your settings later? There's a small ⚙ button in the app for that, anytime.

## Credit

This app is just a friendly face on top of [**iso2god-rs**](https://github.com/iliazeus/iso2god-rs) by iliazeus — that project does all the real conversion work. Massive credit to them; go check it out.

---

<details>
<summary>Building from source (for developers)</summary>

Requires [Rust](https://rustup.rs/) and the Tauri CLI (`cargo install tauri-cli --version "^2.0"`).

```sh
git clone https://github.com/CamiBurger/iso2god-gui.git
cd iso2god-gui
cargo tauri build
```

The `iso2god-rs` CLI binaries for each platform are already bundled in `src-tauri/binaries/` (pulled from the [iso2god-rs releases](https://github.com/iliazeus/iso2god-rs/releases)), so no extra setup is needed to build.

This repo is MIT licensed — see [LICENSE](LICENSE). That covers this GUI wrapper only; `iso2god-rs` is licensed separately by its own author.

</details>
