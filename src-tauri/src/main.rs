// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handler;

use pandoc::OutputKind;

// convert markdown
#[tauri::command]
fn convert(name: &str) -> String {
    let mut pandoc = pandoc::new();
    pandoc.add_input(name);
    pandoc.set_output(OutputKind::Pipe);

    let res = pandoc.execute().unwrap();

    let mut nvim_handler = handler::NvimHandler::new();
    nvim_handler.write_to_buffer("Hello from Rust!");

    match res {
        pandoc::PandocOutput::ToFile(_) => todo!(),
        pandoc::PandocOutput::ToBuffer(s) => {
            println!("{}", s);
            return s
        },
        pandoc::PandocOutput::ToBufferRaw(_) => todo!(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
