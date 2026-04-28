use std::path::PathBuf;
use crate::binary;
use crate::ui;

pub fn execute(path: PathBuf) -> anyhow::Result<()> {
    let info = binary::load_and_analyze(&path)?;
    ui::tui::run_tui(info)?;
    Ok(())
}
