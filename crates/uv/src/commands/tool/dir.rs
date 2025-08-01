use anstream::println;
use anyhow::Context;
use owo_colors::OwoColorize;

use uv_configuration::Preview;
use uv_fs::Simplified;
use uv_tool::{InstalledTools, tool_executable_dir};

/// Show the tool directory.
pub(crate) fn dir(bin: bool, _preview: Preview) -> anyhow::Result<()> {
    if bin {
        let executable_directory = tool_executable_dir()?;
        println!("{}", executable_directory.simplified_display().cyan());
    } else {
        let installed_tools =
            InstalledTools::from_settings().context("Failed to initialize tools settings")?;
        println!("{}", installed_tools.root().simplified_display().cyan());
    }

    Ok(())
}
