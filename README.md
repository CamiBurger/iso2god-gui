# ISO to GOD Converter

A small, cross-platform (macOS / Windows / Linux) GUI wrapper around [`iso2god-rs`](https://github.com/iliazeus/iso2god-rs) — a command-line tool that converts Xbox 360 (and original Xbox) ISOs into the Xbox 360 Games-on-Demand (GOD) format.

This app does not do any of the actual conversion work itself. It just gives you file pickers, a progress bar, and a couple of remembered settings around the real tool, which runs entirely on your machine.

## ⚠️ First launch: unsigned app warning

This app is not code-signed (that costs money — an Apple Developer ID is $99/year, a Windows certificate is $100–400/year — which isn't worth it for a small open-source tool). Your OS will warn you the first time you open it. This is normal, and it only happens once.

**macOS:** Right-click (or Control-click) the app → **Open** → **Open** again in the dialog that appears.
Alternatively: **System Settings → Privacy & Security**, scroll down, and click **Open Anyway**.

**Windows:** Click **More info** on the SmartScreen popup, then **Run anyway**.

**Linux:** No warning — just mark the AppImage/binary executable (`chmod +x`) and run it.

## What it does

1. First time you run it, it asks for your default settings: worker thread count, trim mode, and whether you want to be asked every time or just once.
2. If you chose "just once," future runs skip straight to picking files — no interruptions between selecting your ISOs and starting the conversion.
3. You can reopen settings anytime via the ⚙ button in the app — it's not hidden behind a first-run-only flow.
4. Pick one or more `.iso` files, pick a destination folder, hit **Convert**.

## Download

Grab the build for your OS from the [Releases](../../releases) page.

## Building from source

Requires [Rust](https://rustup.rs/) and the [Tauri CLI](https://tauri.app/) (`cargo install tauri-cli --version "^2.0"`).

```sh
git clone https://github.com/CamiBurger/iso2god-gui.git
cd iso2god-gui
cargo tauri build
```

The `iso2god-rs` CLI binaries for each platform are bundled as sidecars in `src-tauri/binaries/` (pulled from the [iso2god-rs releases](https://github.com/iliazeus/iso2god-rs/releases)) — no separate install step needed.

## Credits

All of the actual ISO → GOD conversion logic belongs to **[iliazeus/iso2god-rs](https://github.com/iliazeus/iso2god-rs)**, licensed under MIT. This project just wraps it in a GUI. Full credit to iliazeus for the tool that does the real work.

## License

MIT — see [LICENSE](LICENSE). This covers the GUI wrapper code in this repository only; `iso2god-rs` is separately licensed by its own author (see link above).
