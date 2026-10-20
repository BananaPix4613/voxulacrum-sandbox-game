//! Voxbox, a prototype game on the voxulacrum engine. It starts as a copy of the
//! engine's sandbox world, run as a game outside the engine repository, and is
//! built on the `voxulacrum` facade alone.

use std::path::PathBuf;

fn main() {
    // Anchored to this crate at compile time, so the game finds its package from
    // any working directory.
    let content = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
    voxulacrum::run_with(voxulacrum::RunOptions::new().package(content));
}
