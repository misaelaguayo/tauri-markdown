use std::sync::mpsc::Sender;

use neovim_lib::{Neovim, NeovimApi, Session, UiAttachOptions};
use log::info;

pub struct NvimHandler {
    nvim: Neovim,
    sender: Sender<String>,
}

impl NvimHandler {
    pub fn new(sender: Sender<String>) -> Self {
        let session = Session::new_tcp("127.0.0.1:6666").unwrap();
        let nvim = Neovim::new(session);

        NvimHandler { nvim, sender }
    }

    pub fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        let current_buffer = self.nvim.get_current_buf().unwrap();

        // Attach current buffer to the ui
        current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();

        // Attach to UI just to get redraw notification, so we make sure every option is deactivated
        let mut ui_options = UiAttachOptions::new();
        ui_options.set_tabline_external(false);
        ui_options.set_cmdline_external(false);
        ui_options.set_hlstate_external(false);
        ui_options.set_linegrid_external(false);
        ui_options.set_tabline_external(false);
        ui_options.set_popupmenu_external(false);
        ui_options.set_rgb(false);
        ui_options.set_wildmenu_external(false);
        self.nvim.ui_attach(100, 100, &ui_options).unwrap();


        println!("Starting event loop");

        for (event, _) in receiver {
            info!("received rpc message : {}", event.clone());
            println!("Event: {}", event);
            match event.as_str() {
                "redraw" => {},
                "nvim_buf_lines_event" => {
                    let buff_string = self.curr_buff_to_string();
                    let _ = self.sender.send(buff_string);
                },
                "nvim_buf_detach_event" => {},
                "rust_doc_open" => {},
                "lock" => {},
                _ => {},
            }
        }
    }

    fn curr_buff_to_string(&mut self) -> String {
        let buffer = self.nvim.get_current_buf().unwrap();
        let total_lines = buffer.line_count(&mut self.nvim).unwrap();
        buffer
            .get_lines(&mut self.nvim, 0, total_lines, true)
            .unwrap()
            .iter()
            .map(|line| format!("{}\n", line.to_owned()))
            .collect::<String>()
    }

    // test function which writes to buffer
    pub fn write_to_buffer(&mut self, text: &str) {
        let buffer = self.nvim.get_current_buf().unwrap();
        buffer.set_lines(&mut self.nvim, 0, -1, true, vec![text.to_string()]).unwrap();
    }
}
