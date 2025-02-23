use crate::UciError;
use crate::UciResult;

#[derive(Debug, Clone)]
pub enum Score {
    Cp(i32),
    Mate(i32),
}

impl Default for Score {
    fn default() -> Self {
        Score::Cp(0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct UciInfo {
    pub depth: u32,
    pub score: Score,
    pub pv: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct GoCommandResult {
    pub best_move: Option<String>,
    pub infos: Vec<UciInfo>,
}

impl GoCommandResult {
    pub fn from_str(input: &str) -> UciResult<GoCommandResult> {
        let mut result = GoCommandResult::default();

        for line in input.lines() {
            if line.starts_with("bestmove") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                result.best_move = parts.get(1).map(|v| v.to_string());
            }

            if line.starts_with("info depth") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                let mut info = UciInfo::default();

                info.depth = parts
                    .get(2)
                    .and_then(|v| v.parse().ok())
                    .ok_or(UciError::EngineError("Unable to parse depth".to_string()))?;

                let mut i = 3;
                while i < parts.len() {
                    match parts[i] {
                        "score" => {
                            match parts[i + 1] {
                                "cp" => {
                                    info.score = Score::Cp(parts[i + 2].parse().map_err(|_| {
                                        UciError::EngineError("Unable to parse score".to_string())
                                    })?);

                                    i += 2;
                                }
                                "mate" => {
                                    info.score =
                                        Score::Mate(parts[i + 2].parse().map_err(|_| {
                                            UciError::EngineError(
                                                "Unable to parse score".to_string(),
                                            )
                                        })?);

                                    i += 2;
                                }
                                _ => {
                                    return Err(UciError::EngineError(
                                        "Unknown score type".to_string(),
                                    ));
                                }
                            }
                            i += 1;
                        }
                        "pv" => {
                            info.pv = parts[i + 1..]
                                .to_vec()
                                .into_iter()
                                .map(|v| v.to_string())
                                .collect();

                            break;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }

                result.infos.push(info);
            }
        }

        Ok(result)
    }

    pub fn best_info(&self) -> UciResult<&UciInfo> {
        self.infos.last().ok_or(UciError::EngineError(
            "There is no info from this go command".to_string(),
        ))
    }
}
