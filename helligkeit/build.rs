use std::path::PathBuf;

use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

fn render_man_page(root: &PathBuf, name: &'static str, cmd: clap::Command) -> std::io::Result<()> {
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    let mut file_name = if name.is_empty() {
        cmd.get_name().to_string()
    } else {
        let mut f = name.to_string();
        f.push('-');
        f.push_str(cmd.get_name());
        f
    };
    file_name.push_str(".1");
    std::fs::write(root.join(file_name), buffer)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let mut project_root = std::path::PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").ok_or(std::io::ErrorKind::NotFound)?,
    );

    project_root = project_root.join("..").join("man");
    if project_root.exists() {
        std::fs::remove_dir_all(&project_root)?;
    }
    std::fs::create_dir_all(&project_root)?;

    let cmd = cli::Cli::command();
    render_man_page(&project_root, "", cmd)?;

    for sub_cmd in cli::Cli::command().get_subcommands() {
        let cmd_name = sub_cmd.get_name();
        if matches!(cmd_name, "help") {
            continue;
        }
        render_man_page(&project_root, "helligkeit", sub_cmd.clone())?;
    }

    Ok(())
}
