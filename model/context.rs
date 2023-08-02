use std::path::PathBuf;
use std::io::{Read, Result as IOResult};
use std::sync::{Mutex, Arc};

use winit::event_loop::{EventLoopProxy, EventProxy};
use winit::event::Event as WinitEvent;

use alacritty_terminal::tty::{self, Pty};
use alacritty_terminal::term::test::TermSize;
use alacritty_terminal::term::Term;
use alacritty_terminal::config::{PtyConfig, Config};
use alacritty_terminal::event::{WindowSize, Event};


pub struct Context {
    pub x: i128,
    pub dropdown_selected: usize,
    pub term_text: String,
    pub tty: Pty,
    pub event_loop: EventLoopProxy<Event>
}

fn ts_to_ws (ts: &TermSize) -> WindowSize {
    return WindowSize {
        num_lines: ts.screen_lines as u16,
        num_cols: ts.columns as u16, 
        cell_width: 2,
        cell_height: 4,
    }
}

impl Context {
    pub fn new (x: i128, event_loop: EventLoopProxy<Event>) -> Self {
        let event_proxy = EventProxy::new(event_loop, 1);

        let size_info = TermSize { columns: 80, screen_lines: 20 };

        let terminal = Term::new(
            &Config::default(),
            &size_info,
            event_proxy.clone()
        );
        let terminal = Arc::new(Mutex::new(terminal));

        let pty = tty::new(
            &PtyConfig::default(), 
            ts_to_ws(&size_info),
            1
        ).expect("Should be able to instantiate pty");


        Self {
            x,
            dropdown_selected: 0,
            term_text: String::from(""),
            tty: pty,
            event_loop
        }
    }

    pub fn update (&mut self) -> IOResult<usize> {
        self.tty.file().read_to_string(&mut self.term_text)
    }
}
