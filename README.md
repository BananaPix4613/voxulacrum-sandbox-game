# Voxbox

A prototype game on the [voxulacrum](https://github.com/BananaPix4613/voxulacrum)
engine. It starts as a copy of the engine's sandbox world, run as a game outside
the engine repository, and depends on the engine's `voxulacrum` facade alone.

| Path | What |
|---|---|
| `src/main.rs` | Starts the engine with this repository's package |
| `content/` | The `voxbox` package: `package.ron`, `play.ron` and the world's content |
| `.cargo/config.example.toml` | The local engine patch, for co-development |

## Building

`cargo run --release` builds against the engine's git repository.

To build against a local engine checkout beside this one (`../voxulacrum`), copy
`.cargo/config.example.toml` to `.cargo/config.toml`. That copy is gitignored,
so CI always builds against the git dependency.

The engine finds its shaders, palettes and input bindings in its own checkout,
and keeps this game's saves there too, under `saves/voxbox/`.

## Checks

```powershell
cargo run --release -- --validate-graphs
```
```powershell
cargo run --release -- --verify-generation 64
```
```powershell
cargo install --git https://github.com/BananaPix4613/voxulacrum boundary-check
```
```powershell
boundary-check --manifest-path Cargo.toml
```

CI runs all of them on every push.

## Content license

`content/` started as a copy of the engine's sandbox package, which is licensed
under CC BY 4.0 (see the engine's `assets/LICENSE`).
