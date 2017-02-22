extern crate pty_shell;

use pty_shell::*;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::process::{Command,Stdio};

struct Shell;
impl PtyHandler for Shell {
    fn input(&mut self, input: &[u8]) {
        playse("se-type.wav");
    }

    fn output(&mut self, output: &[u8]) {
        // do something with output
    }

    fn resize(&mut self, winsize: &winsize::Winsize) {
        playse("se-type.wav");
    }

    fn shutdown(&mut self) {
        playse("se-end.wav");
    }
}

fn playse<S: AsRef<Path>>(file: S) {
    let path = Path::new("/tmp/ashell/sounds");
    let _ = Command::new("aplay")
                     .arg(path.join(file).to_str().unwrap())
                     .stdout(Stdio::null())
                     .stderr(Stdio::null())
                     .spawn();
}

fn shell() -> String {
    match env::var("SHELL") {
        Ok(shell) => shell,
        Err(_)    => "bash".to_string()
    }
}

fn invoke_ashell() {
    let child = tty::Fork::from_ptmx().unwrap();
    let shell = shell();

    let _ = child.exec(&shell);

    playse("se-chon.wav");

    if let Some(master) = child.is_parent().ok() {
        let mut writer = master.clone();
        let command = format!(
            "source shell_extensions/ashell.{}\n",
            Path::new(&shell).file_name().unwrap().to_str().unwrap()
        );

        let _ = writer.write(command.as_bytes());
    }

    let _ = child.proxy(Shell);
    let _ = child.wait();
}

fn setup_sounds() {
    let path = Path::new("/tmp/ashell/sounds");

    if path.exists() {
        return;
    }

    fs::create_dir_all(path).unwrap();

    let _ = fs::File::create(path.join("se-chdir.wav")).unwrap().write_all(include_bytes!("../sounds/se-chdir.wav"));
    let _ = fs::File::create(path.join("se-end.wav")).unwrap().write_all(include_bytes!("../sounds/se-end.wav"));
    let _ = fs::File::create(path.join("se-failed.wav")).unwrap().write_all(include_bytes!("../sounds/se-failed.wav"));
    let _ = fs::File::create(path.join("se-preexec.wav")).unwrap().write_all(include_bytes!("../sounds/se-preexec.wav"));
    let _ = fs::File::create(path.join("se-start.wav")).unwrap().write_all(include_bytes!("../sounds/se-start.wav"));
    let _ = fs::File::create(path.join("se-succeeded.wav")).unwrap().write_all(include_bytes!("../sounds/se-succeeded.wav"));
    let _ = fs::File::create(path.join("se-type.wav")).unwrap().write_all(include_bytes!("../sounds/se-type.wav"));
}

fn main() {
    setup_sounds();
    invoke_ashell();
}
