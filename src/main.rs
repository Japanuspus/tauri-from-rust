
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

fn main() {
    let state = MyState{count: 0};
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![my_custom_command, elaborate_command])
        .run(tauri::generate_context!())
        .expect("unable to run Tauri application");
}
