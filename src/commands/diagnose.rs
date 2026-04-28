use std::path::PathBuf;
use crate::binary;
use crate::ui;
use crate::analysis;

pub fn execute(path: PathBuf) -> anyhow::Result<()> {
    let info = binary::load_and_analyze(&path)?;
    let diags = analysis::run_diagnostics(&info);
    ui::display_diagnostics(&diags);
    Ok(())
}
