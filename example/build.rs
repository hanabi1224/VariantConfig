use std::process::Command;

// https://doc.rust-lang.org/cargo/reference/environment-variables.html
fn main() -> anyhow::Result<()> {
    let cwd = "pages";
    #[cfg(not(windows))]
    let cmd = "yarn";
    #[cfg(windows)]
    let cmd = "yarn.cmd";
    Command::new(cmd)
        .current_dir(cwd)
        .spawn()
        .expect("yarn failed");
    Command::new(cmd).args(["build"]).current_dir(cwd).spawn()?;
    Ok(())
}
