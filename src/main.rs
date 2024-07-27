use std::fmt::Debug;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use clap::Parser;

pub mod cli;
use cli::Cli;

#[derive(Debug)]
pub enum Status {
    UptoDate { path: PathBuf },
    BranchAhead { path: PathBuf, buffer: String },
    ChangesNotStaged { path: PathBuf, buffer: String },
    UntrackedFiles { path: PathBuf, buffer: String },
    NotAGitRepository { path: PathBuf, buffer: String },
    Other { path: PathBuf, buffer: String },
}

#[derive(Debug, Default)]
pub struct StatusSummary {
    up_to_date: Vec<PathBuf>,
    branch_ahead: Vec<PathBuf>,
    changes_not_staged: Vec<PathBuf>,
    untracked_files: Vec<PathBuf>,
    other: Vec<PathBuf>,
}

impl StatusSummary {
    fn increment(&mut self, status: &Status) {
        match status {
            Status::UptoDate { path } => self.up_to_date.push(path.clone()),
            Status::BranchAhead { path, buffer: _ } => self.branch_ahead.push(path.clone()),
            Status::UntrackedFiles { path, buffer: _ } => self.untracked_files.push(path.clone()),
            Status::NotAGitRepository { path: _, buffer: _ } => {}
            Status::Other { path, buffer: _ } => self.other.push(path.clone()),
            Status::ChangesNotStaged { path, buffer: _ } => {
                self.changes_not_staged.push(path.clone())
            }
        }
    }

    pub fn short(&self) {
        println!("UptoDate: {}", self.up_to_date.len());
        println!("BranchAhead: {}", self.branch_ahead.len());
        println!("UntrackedFiles: {}", self.untracked_files.len());
        println!("Other: {}", self.other.len());
        println!("ChangesNotStaged: {}", self.changes_not_staged.len());
    }

    pub fn long(&self) {
        println!("--- UptoDate ---\n\n");
        for entry in &self.up_to_date {
            println!("{entry:?}");
        }
        println!("--- BranchAhead ---\n\n");
        for entry in &self.branch_ahead {
            println!("{entry:?}");
        }
        println!("--- UntrackedFiles ---\n\n");
        for entry in &self.untracked_files {
            println!("{entry:?}");
        }
        println!("--- Other ---\n\n");
        for entry in &self.other {
            println!("{entry:?}");
        }
        println!("--- ChangesNotStaged ---\n\n");
        for entry in &self.changes_not_staged {
            println!("{entry:?}");
        }
    }
}

impl Status {
    pub fn new(path: PathBuf) -> Self {
        let buffer: String = get_status(&path);
        if buffer.contains("Your branch is ahead of") {
            return Self::BranchAhead { path, buffer };
        }
        if buffer.contains("Changes not staged for commit:") {
            return Self::ChangesNotStaged { path, buffer };
        }
        if buffer.contains("Untracked files:") {
            return Self::UntrackedFiles { path, buffer };
        }
        if buffer.contains("Your branch is up to date with") {
            return Self::UptoDate { path };
        }
        if buffer.contains("fatal: not a git repository ") {
            return Self::NotAGitRepository { path, buffer };
        }
        Self::Other { path, buffer }
    }

    pub(crate) fn search(
        parent_path: PathBuf,
        cli: &Cli,
        summary: &Arc<Mutex<StatusSummary>>,
    ) -> io::Result<()> {
        let entries: fs::ReadDir = fs::read_dir(&parent_path)?;
        let mut directories: Vec<PathBuf> = Vec::new();
        for entry in entries {
            let entry: fs::DirEntry = entry?;
            let path: PathBuf = entry.path();

            if path.is_dir() {
                directories.push(path.clone());
            }

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name == "Cargo.toml" {
                        let status: Self = Self::new(parent_path.clone());
                        if cli.debug {
                            println!("{status:?}");
                        }
                        summary.lock().unwrap().increment(&status);
                        return Ok(());
                    }
                }
            }
        }
        for directory in directories {
            Self::search(directory, cli, summary)?;
        }
        Ok(())
    }
}

fn get_status(path: &PathBuf) -> String {
    let output = Command::new("git")
        .arg("status")
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdout = output.stdout.expect("Failed to capture stdout");
    let mut stderr = output.stderr.expect("Failed to capture stderr");

    let mut buffer: String = String::new();
    stdout
        .read_to_string(&mut buffer)
        .expect("Failed to capture stdout");

    let mut error_buffer: String = String::new();
    stderr
        .read_to_string(&mut error_buffer)
        .expect("failed to read stderr");

    if !error_buffer.is_empty() {
        return error_buffer;
    }

    buffer
}

fn main() {
    let cli: Cli = Cli::parse();
    let path: &String = &cli.path;
    let path: PathBuf = PathBuf::from_str(path).unwrap();
    let summary: StatusSummary = StatusSummary::default();
    let summary: Arc<Mutex<StatusSummary>> = Arc::new(Mutex::new(summary));

    Status::search(path, &cli, &summary).unwrap();
    if cli.long {
        summary.lock().unwrap().long();
    } else {
        summary.lock().unwrap().short();
    }
}
