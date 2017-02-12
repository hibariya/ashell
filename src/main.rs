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
        playse("se-chalk.wav");
    }

    fn output(&mut self, output: &[u8]) {
        // do something with output
    }

    fn resize(&mut self, winsize: &winsize::Winsize) {
        playse("se-awa.wav");
    }

    fn shutdown(&mut self) {
        playse("se-chon.wav");
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

    let se_awa = include_bytes!("../sounds/se-awa.wav");
    let mut se_awa_f = fs::File::create(path.join("se-awa.wav")).unwrap();
    let _ = se_awa_f.write_all(se_awa);

    let se_awa = include_bytes!("../sounds/se-chalk.wav");
    let mut se_awa_f = fs::File::create(path.join("se-chalk.wav")).unwrap();
    let _ = se_awa_f.write_all(se_awa);

    let se_awa = include_bytes!("../sounds/se-chon.wav");
    let mut se_awa_f = fs::File::create(path.join("se-chon.wav")).unwrap();
    let _ = se_awa_f.write_all(se_awa);

    let se_awa = include_bytes!("../sounds/se-kabe.wav");
    let mut se_awa_f = fs::File::create(path.join("se-kabe.wav")).unwrap();
    let _ = se_awa_f.write_all(se_awa);
}

fn main() {
    setup_sounds();
    invoke_ashell();
}
