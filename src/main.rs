use tauri::{Window, Manager}; 

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
    // state.count+=1;  // wrap in mutex for mutability
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


fn main() {
    let state = MyState{count: 0};
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
          // The `.get_window`-method is part of the `Manager`-trait
          let main_window = app.get_window("main").unwrap();
          std::thread::spawn(move || window_pinger(main_window));
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command, elaborate_command])
        .run(tauri::generate_context!())
        .expect("unable to run Tauri application");
}
