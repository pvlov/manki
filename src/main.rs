mod deck;
mod file_manager;
mod gui;
mod gui_util;
mod icons;
mod logger;

use crate::deck::Deck;
use crate::logger::Logger;
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::Context;

pub enum State {
    HOMESCREEN,
    STUDYSCREEN,
}

struct Manki {
    state: State,
    curr_deck: Deck, //current deck, either the one currently being studied, edited or created
    index: usize,
    _logger: Logger,
}

impl Manki {
    fn default(_cc: &CreationContext<'_>) -> Manki {
        return Manki {
            state: State::HOMESCREEN,
            curr_deck: Deck::empty("Empty"),
            index: 0,
            _logger: Logger::new(),
        };
    }
}

impl App for Manki {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.info().window_info;

        match &self.state {
            State::HOMESCREEN => gui::render_homescreen(ctx, self),
            State::STUDYSCREEN => gui::render_studyscreen(ctx, self),
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();
    run_native(
        "Manki",
        native_options,
        Box::new(|cc| Box::new(Manki::default(cc))),
    )
}
