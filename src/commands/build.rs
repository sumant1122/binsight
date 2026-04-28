use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::commands;
use colored::*;

pub fn execute(manifest_path: Option<PathBuf>) -> anyhow::Result<()> {
    println!("🚀 {} project with timings...", "Building".cyan().bold());

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release", "--timings", "--message-format=json"]);
    
    if let Some(path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }

    let mut child = cmd.stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut binaries = Vec::new();
    let reader = BufReader::new(child.stdout.take().unwrap());

    for line in reader.lines() {
        let line = line?;
        // Simple JSON parsing to find executables
        if line.contains("\"executable\"") && line.contains("\"target\"") {
            if let Some(exec_path) = parse_executable_path(&line) {
                binaries.push(PathBuf::from(exec_path));
            }
        }
    }

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("Build failed");
    }

    if binaries.is_empty() {
        println!("{}", "No executable artifacts found.".yellow());
        return Ok(());
    }

    println!("\n✅ {} complete. Analyzing {} binaries...\n", "Build".green().bold(), binaries.len());

    for bin in binaries {
        println!("🔍 Analyzing {}...", bin.display().to_string().magenta());
        commands::analyze::execute(bin)?;
        println!("\n---\n");
    }

    println!("ℹ️  Detailed build timings available in {}/", "target/cargo-timings".cyan());

    Ok(())
}

fn parse_executable_path(line: &str) -> Option<String> {
    // This is a quick and dirty JSON extractor to avoid adding a heavy serde_json dependency
    // specifically for this one command. It looks for "executable":"PATH"
    if let Some(idx) = line.find("\"executable\":\"") {
        let start = idx + 14;
        if let Some(end) = line[start..].find('\"') {
            return Some(line[start..start+end].replace("\\\\", "\\"));
        }
    }
    None
}
