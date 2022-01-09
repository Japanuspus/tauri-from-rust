use tauri::{AppHandle, Manager, Window, http, GlobalShortcutManager, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent}; 

// Wrap in mutex for mutability:
// https://docs.rs/tauri/1.0.0-beta.8/tauri/struct.Builder.html#mutability
// https://github.com/tauri-apps/tauri/blob/next/examples/state/src-tauri/src/main.rs
struct MyState {
    count: usize
}

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  other_val: usize,
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn elaborate_command(
  window: tauri::Window,
  message: String, 
  state: tauri::State<'_, MyState>,
) -> Result<CustomResponse, String> {
    println!("Called from {} with message \"{}\"", window.label(), &message);
    window.emit("my-event", CustomResponse {message: format!("Backend received: {}", &message), other_val: 0}).unwrap();

    // state.count+=1;  // wrap in mutex for mutability, see above
    Ok(CustomResponse{message, other_val: state.count})
}

fn window_pinger(window: Window) {
  let mut i:u64=0;
  loop {
    window.emit("my-event", CustomResponse { message: "Tick from backend".into(), other_val: i as usize}).unwrap();
    println!("Emitting my-event to window - {}.", i);
    std::thread::sleep(std::time::Duration::from_secs(i+1));
    i=(i+1)%60;
  }
}


// https://github.com/tauri-apps/wry/blob/dev/examples/custom_protocol.rs
// https://docs.rs/tauri/1.0.0-beta.8/tauri/struct.Builder.html#method.register_uri_scheme_protocol
fn my_protocol(_app: &AppHandle, _request: &http::Request) ->  Result<http::Response, Box<dyn std::error::Error>>  {
  http::ResponseBuilder::new()
  .status(202)
  .mimetype("application/json")
  .body(r#"{"a": 3, "b": 4}"#.into())
}


fn hotkey_callback(app: &AppHandle) {
  app.emit_all("string-event", "Hotkey pressed").unwrap();
  println!("Global hotkey pressed");
}


fn main() {
    let state = MyState{count: 0};

    // System tray requires config entry and feature "system-tray", see
    // https://tauri.studio/en/docs/usage/guides/visual/system-tray/
    // Good example on window lifetime mgmt in 
    // see https://github.com/tauri-apps/tauri/blob/dev/examples/api/src-tauri/src/main.rs 
    let system_tray = SystemTray::new().with_menu(
      SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("ping", "Ping"))
    );

    // docs at https://docs.rs/tauri/1.0.0-beta.8/tauri/struct.Builder.html
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
          // The `.get_window`-method is part of the `Manager`-trait
          let main_window = app.get_window("main").unwrap();
          std::thread::spawn(move || window_pinger(main_window));
          
          let mut shortcut_manager = app.global_shortcut_manager();
          let app_handle = app.handle();
          let handler = move || hotkey_callback(&app_handle);
          std::thread::spawn(move || shortcut_manager.register("COMMANDORCONTROL+SHIFT+3", handler));
          Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
          let msg = match event {
            SystemTrayEvent::LeftClick {
              position: _,
              size: _,
              ..
            } => "Systray Left Click",
            SystemTrayEvent::MenuItemClick { id, .. } => match &id.to_string()[..] {
                "ping" => "Systray Ping",
                _ => panic!("Unexpected menu id: {}", id),
            },
            _ => "Systray Other",
          };
          app.emit_all("string-event", msg).unwrap();
        })
        // does not appear to be working on windows for 1.0.0beta8
        .register_uri_scheme_protocol("example", my_protocol)
        .invoke_handler(tauri::generate_handler![my_custom_command, elaborate_command])
        .run(tauri::generate_context!())
        .expect("unable to run Tauri application");
}
