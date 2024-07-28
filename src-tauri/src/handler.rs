use neovim_lib::{Neovim, NeovimApi, Session, UiAttachOptions};

pub struct NvimHandler {
    nvim: Neovim,
}

impl NvimHandler {
    pub fn new() -> Self {
        let session = Session::new_tcp("127.0.0.1:6666").unwrap();
        let mut nvim = Neovim::new(session);
        // start event loop
        nvim.session.start_event_loop_channel();

        NvimHandler { nvim }
    }

    // test function which writes to buffer
    pub fn write_to_buffer(&mut self, text: &str) {
        let buffer = self.nvim.get_current_buf().unwrap();
        buffer.set_lines(&mut self.nvim, 0, -1, true, vec![text.to_string()]).unwrap();
    }
}
