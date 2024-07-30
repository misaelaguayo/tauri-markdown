// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::mpsc::channel, thread};

use pandoc::OutputKind;
use tauri::Manager;

mod handler;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn test_app_handle(app: tauri::AppHandle) {
    println!("Sending message to all windows");
    app.emit_all("test", Payload { message: "Tauri is awesome".into() }).unwrap();
}

// convert markdown
#[tauri::command]
fn convert(name: &str) -> String {
    let (sender, receiver) = channel();

    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(name.to_string()));
    pandoc.set_output(OutputKind::Pipe);

    let res = pandoc.execute().unwrap();

    let mut nvim_handler = handler::NvimHandler::new(sender);

    thread::spawn(move || {
        nvim_handler.recv();
    });

    thread::spawn(move || {
        for received in receiver {
            match received.as_str() {
                "buf_lines" => {}
                _ => {}
            }
        }
    });

    match res {
        pandoc::PandocOutput::ToFile(_) => todo!(),
        pandoc::PandocOutput::ToBuffer(s) => {
            println!("{}", s);
            return s;
        }
        pandoc::PandocOutput::ToBufferRaw(_) => todo!(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_app_handle, convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
