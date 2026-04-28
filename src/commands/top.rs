use std::path::PathBuf;
use crate::binary;
use crate::ui;

pub fn execute(path: PathBuf, depth: usize) -> anyhow::Result<()> {
    let info = binary::load_and_analyze(&path)?;
    ui::display_top_contributors(&info, depth);
    Ok(())
}
