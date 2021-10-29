fn main() {
    // from https://tauri.studio/en/docs/usage/guides/webdriver/example/setup

    // Only watch the `dist/` directory for recompiling, preventing unnecessary
    // changes when we change files in other project subdirectories.
    println!("cargo:rerun-if-changed=dist");

    // Run the Tauri build-time helpers
    tauri_build::build()
}