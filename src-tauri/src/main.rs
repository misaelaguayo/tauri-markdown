// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handler;

use std::thread;
use tauri::async_runtime::block_on;
use tokio::sync::mpsc;

use pandoc::OutputKind;

// convert markdown
#[tauri::command]
fn convert(name: &str) -> String {
    let (sender, mut receiver) = mpsc::channel::<String>(1);

    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(name.to_string()));
    pandoc.set_output(OutputKind::Pipe);

    let res = pandoc.execute().unwrap();

    let mut nvim_handler = handler::NvimHandler::new(sender);

    thread::spawn(move || {
        block_on(nvim_handler.recv());

        block_on(async {
            while let Some(msg) = receiver.recv().await {
                println!("Received: {}", msg);
            }
        });
    });

    // nvim_handler.write_to_buffer("Hello from Rust!");

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
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
