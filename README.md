# Tauri frontend in a pure rust project

A minimal example of building a [tauri] "Hello world" app entirely from [rust], without installing or using any js-tools.

[tauri] is a project that facilitates writing cross-platform apps based a web-technology frontend running in a windowed web-view.
The official docs and templates are aimed at people experienced in the web-frontend ecosystem, i.e. the rust (backend) code is located in a subproject inside the front-end project and the build system entry-point is the frontend build system (`yarn dev`).

The purpose of this project is to demonstrate how to use [tauri] in a rust project without any front-end build system.
This is suitable for projects using vanilla html/js frontends, but mostly intended as a way to play with [tauri] for rustaceans without getting distracted by the frontend tooling.
If you are using a web framework for frontend (or prefer `yarn dev` over `cargo run`), you should rather look at the official examples which also provide hot-reload and other niceties.

### Alternatives to [tauri]
Compared to [electron]-apps, [tauri] replaces the backed node server with a rust application -- which is nice if you are a rust person.
More importantly, application size is significantly reduced by using platform-native web front-ends (via [webview]) rather than shipping a full chrome application.

Compared to using [webview] directly, [tauri] adds some complexity and a lot of functionality and convenience:
- Window menu
- Global hotkey
- Tray icon
- App icon
- Cross-platform builds

In addition, [tauri] provides a js-API to allow the frontend do directly access backend functionality. This is not something that I will explore in this project.

### Running
Install the [rust] toolchain, then

    cargo run

### TODO

- [x] Demonstrate a "Hello world"-application
- [x] Demonstrate call from js (front-end) to rust (back-end)
- [x] Demonstrate signal from rust to js
- [ ] Demonstrate custom protocol
- [x] Demonstrate tray icon
- [x] Demonstrate global hotkey
- [ ] Set up CI on github

[rust]: https://www.rust-lang.org/
[tauri]: https://tauri.studio/en/
[electron]: https://www.electronjs.org/
[webview]: https://github.com/webview
