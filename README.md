<div align="center">
  <h2>En Croissant + Maia 1700</h2>
  <p><strong>En Croissant</strong> (the Ultimate Chess Toolkit) pre-bundled with the
  <a href="https://github.com/CSSLab/maia-chess"><strong>Maia 1700</strong></a> human-like chess engine for Android.</p>
  <p>Play against a neural network trained on the average moves of 1700-ELO Lichess players, fully offline.</p>
</div>

## What's included

- The full [En Croissant](https://github.com/franciscoBSalgueiro/en-croissant) chess GUI (GPL-3.0).
- [Maia 1700](https://github.com/CSSLab/maia-chess) neural network weights.
- A custom Android build of [lc0](https://github.com/LeelaChessZero/lc0) (v0.30.0, pure-Eigen CPU backend,
  `aarch64` only) acting as the UCI engine that runs Maia.
- A GitHub Actions workflow (`.github/workflows/build-apk.yml`) that compiles lc0 for Android, embeds it in
  the app, and produces an installable APK.

## Install the Android APK

The APK is built automatically on every push to `main` and attached to the workflow run as an artifact
(`en-croissant-maia-android`). You can also grab it from the **Releases** page.

1. Download the APK from the latest workflow run or release.
2. Allow "install from unknown sources" on your device.
3. Open the APK - on first launch the app extracts the bundled lc0 engine and registers **Maia 1700**
   in the engine list.
4. Start a new game, pick an engine opponent and choose **Maia 1700**.

Maia plays the _average_ move of a 1700-rated Lichess player: the GUI configures it with `go nodes 1`
(search is disabled, a single neural network evaluation per move), matching how the Maia models are meant
to be used.

## Building the Android APK from source

GitHub Actions builds the APK in the cloud. To trigger it manually:

```bash
git push origin main
# or click "Run workflow" in the Actions tab
```

Locally, the Android build requires the Android SDK/NDK, Rust's `aarch64-linux-android` target, pnpm and
software like Meson only needed for the one-time lc0 build (which CI also does via
`scripts/build-lc0-android.sh`):

```bash
pnpm install
bash scripts/build-lc0-android.sh   # replaces src-tauri/resources/lc0.gz
pnpm tauri android init --ci
pnpm tauri android build --apk --debug --target aarch64
```

## Engine internals

- The engine binary and weights are embedded into the Rust binary
  (`src-tauri/src/maia.rs`) and extracted on first launch (Android only) to the app's engines directory.
- The extracted `engines/engines.json` seeds the **Maia 1700** local engine entry (
  `WeightsFile`, `Threads`, `NNCacheSize`, `MinibatchSize` options and `go nodes 1`).
- Desktop builds are unchanged; the user manages engines there as usual.

## Credits & licensing

- [En Croissant](https://github.com/franciscoBSalgueiro/en-croissant) - GPL-3.0, by Francisco B. Salgueiro.
- [Maia](https://github.com/CSSLab/maia-chess) - GPL-3.0, by the CSSLab (U of Toronto); "Aligning Superhuman AI
  with Human Behavior: Chess as a Model System" (McIlroy-Young et al., KDD 2020).
- [lc0](https://github.com/LeelaChessZero/lc0) - GPL-3.0, by the Leela Chess Zero team.

---

<br />
<div align="center">
  <a href="https://github.com/franciscoBSalgueiro/en-croissant">
    <img width="115" height="115" src="https://github.com/franciscoBSalgueiro/en-croissant/blob/master/src-tauri/icons/icon.png" alt="Logo">
  </a>

<h3 align="center">En Croissant</h3>

  <p align="center">
    The Ultimate Chess Toolkit
    <br />
    <a href="https://www.encroissant.org"><strong>encroissant.org</strong></a>
    <br />
    <br />
    <a href="https://discord.gg/tdYzfDbSSW">Discord Server</a>
    ·
    <a href="https://www.encroissant.org/download">Download</a>
    .
    <a href="https://www.encroissant.org/docs">Explore the docs</a>
  </p>
</div>

En-Croissant is an open-source, cross-platform chess GUI that aims to be powerful, customizable and easy to use.

## Features

- Store and analyze your games from [lichess.org](https://lichess.org) and [chess.com](https://chess.com)
- Multi-engine analysis. Supports all UCI engines
- Prepare a repertoire and train it with spaced repetition
- Simple engine and database installation and management
- Absolute or partial position search in the database

<img src="https://github.com/franciscoBSalgueiro/encroisssant-site/blob/master/public/showcase.webp" />

## Building from source

Refer to the [Tauri documentation](https://tauri.app/start/prerequisites/) for the requirements on your platform.

En-Croissant uses pnpm as the package manager for dependencies. Refer to the [pnpm install instructions](https://pnpm.io/installation) for how to install it on your platform.

```bash
git clone https://github.com/franciscoBSalgueiro/en-croissant
cd en-croissant
pnpm install
pnpm build
```

The built app can be found at `src-tauri/target/release`

## Donate

If you wish to support the development of this GUI, you can do so [here](https://encroissant.org/support). All donations are greatly appreciated!

## Contributing

For contributing to this project please refer to the [Contributing guide](./CONTRIBUTING.md).

## License

This software is licensed under GPL-3.0 License.
