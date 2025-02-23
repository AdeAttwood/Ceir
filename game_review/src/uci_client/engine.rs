use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

use common::ResolvedMovement;

use crate::UciError;
use crate::UciResult;

use super::GoCommandResult;
use super::UciCommandResult;

#[derive(Debug)]
pub struct UciEngine {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl UciEngine {
    pub fn new(command: &str) -> UciResult<UciEngine> {
        let mut engine_process = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = engine_process.stdin.take().ok_or(UciError::EngineError(
            "Unable to get stdin from the engine process".to_string(),
        ))?;

        let mut stdout = BufReader::new(engine_process.stdout.take().ok_or(
            UciError::EngineError("Unable to get stdout from the engine process".to_string()),
        )?);

        // TODO(AdeAttwood): We don't need the hole first line for the title only the engine name
        let mut first_line = String::new();
        stdout
            .read_line(&mut first_line)
            .map_err(UciError::IoError)?;

        Ok(UciEngine { stdin, stdout })
    }

    pub fn uci(&mut self) -> UciResult<UciCommandResult> {
        writeln!(self.stdin, "{}", "uci").map_err(UciError::IoError)?;
        writeln!(self.stdin, "{}", "isready").map_err(UciError::IoError)?;

        let mut response = String::new();
        for line in self.stdout.by_ref().lines() {
            let line = line.map_err(UciError::IoError)?;
            if line.trim() == "readyok" {
                break;
            }

            response.push_str(&line);
            response.push('\n');
        }

        UciCommandResult::from_str(&response)
    }

    pub fn position(&mut self, fen: &str, moves: &Vec<String>) -> UciResult<()> {
        let mut command = if fen == "startpos" {
            format!("position {}", fen)
        } else {
            format!("position fen {}", fen)
        };

        if !moves.is_empty() {
            command.push_str(" moves ");
            command.push_str(&moves.join(" "));
        }

        writeln!(self.stdin, "{}", command).map_err(UciError::IoError)?;

        Ok(())
    }

    pub fn go_depth(&mut self, depth: u32) -> UciResult<GoCommandResult> {
        writeln!(self.stdin, "go depth {}", depth).map_err(UciError::IoError)?;

        let mut response = String::new();
        for line in self.stdout.by_ref().lines() {
            let line = line.map_err(UciError::IoError)?;
            if line.trim().starts_with("bestmove") {
                response.push_str(&line);
                break;
            }

            response.push_str(&line);
            response.push('\n');
        }

        GoCommandResult::from_str(&response)
    }
}
