use tauri::{AppHandle, Manager, Window, http}; 

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

    // state.count+=1;  // wrap in mutex for mutability, see above
    Ok(CustomResponse{message, other_val: state.count})
}

fn window_pinger(window: Window) {
  let mut i:usize=0;
  loop {
    window.emit("my-event", CustomResponse { message: "Message from backend".into(), other_val: i}).unwrap();
    println!("Emitting my-event to window - {}.", i);
    i+=1;
    std::thread::sleep(std::time::Duration::from_secs(1));
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


fn main() {
    let state = MyState{count: 0};
    // docs at https://docs.rs/tauri/1.0.0-beta.8/tauri/struct.Builder.html
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
          // The `.get_window`-method is part of the `Manager`-trait
          let main_window = app.get_window("main").unwrap();
          std::thread::spawn(move || window_pinger(main_window));
          Ok(())
        })
        // does not appear to be working on windows for 1.0.0beta8
        .register_uri_scheme_protocol("example", my_protocol)
        .invoke_handler(tauri::generate_handler![my_custom_command, elaborate_command])
        .run(tauri::generate_context!())
        .expect("unable to run Tauri application");
}
