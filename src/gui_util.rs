use crate::file_manager::file_name;
use eframe::egui::Button;
use std::path::PathBuf;

pub(crate) struct DeckButton {
    //the absolute path to the deck this button is representing
    pub(crate) path: PathBuf,
    pub(crate) button: Button,
}

impl DeckButton {
    pub(crate) fn from(pth: PathBuf, bttn: Button) -> Self {
        DeckButton {
            path: pth,
            button: bttn,
        }
    }

    pub(crate) fn paths_to_buttons(files: Vec<PathBuf>) -> Vec<DeckButton> {
        return files
            .iter()
            .map(|f| {
                let curr_path: &PathBuf = &f;
                let display: &str = &file_name(&curr_path);
                Self::from(curr_path.to_path_buf(), Button::new(display))
            })
            .collect();
    }
}