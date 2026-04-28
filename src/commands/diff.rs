use std::path::PathBuf;
use crate::binary;
use crate::ui;
use crate::analysis;

pub fn execute(old_path: PathBuf, new_path: PathBuf) -> anyhow::Result<()> {
    let old_info = binary::load_and_analyze(&old_path)?;
    let new_info = binary::load_and_analyze(&new_path)?;
    let diff = analysis::compare(&old_info, &new_info);
    ui::display_diff(&diff);
    Ok(())
}
