# Tauri from rust

A minimal example of building a [tauri] "Hello world" app entirely from [rust], without installing or using any js-tools.

[tauri] is a project that facilitates writing cross-platform apps based a web-technology frontend running in a windowed web-view.
The official docs are aimed at people experienced with the web-frontend ecosystem, and the purpose of this project is to demonstrate how to integrate [tauri] into a project without any external dependencies.
If you are well-versed in web frameworks (or prefer `yarn dev` over `cargo run`), you should rather look at the official examples.

Compared to [electron]-apps, [tauri] replaces the backed node server with a rust application -- which is nice if you are a rust person. More importantly, application size is significantly reduced by using platform-native web front-ends rather than shipping a full chrome application.

Compared to using [webview] directly, [tauri] adds some complexity and a lot of functionality:
- Window menu
- Global hotkey
- Tray icon
- App icon
- Cross-platform builds

In addition, [tauri] provides js-interfaces to backend functionality. This is not something that I will explore in this project.

### TODO

- [x] Demonstrate a "Hello world"-application
- [ ] Demonstrate call from js (front-end) to rust (back-end)
- [ ] Demonstrate signal from rust to js
- [ ] Demonstrate tray icon
- [ ] Demonstrate global hotkey

[rust]: https://www.rust-lang.org/
[tauri]: https://tauri.studio/en/
[electron]: https://www.electronjs.org/
[webview]: https://github.com/webview