#[derive(Default)]
pub struct ArgBuilder {
    args: Vec<String>,
}

impl ArgBuilder {
    pub fn new(args: Vec<String>) -> Self {
        ArgBuilder { args }
    }

    pub fn bool(&self, short: &str, long: &str) -> bool {
        for arg in &self.args {
            if arg == short || arg == long {
                return true;
            }
        }

        false
    }

    pub fn string(&self, short: &str, long: &str) -> Result<String, String> {
        for i in 0..self.args.len() {
            if self.args[i] == short || self.args[i] == long {
                let value = self.args.get(i + 1);
                return match value {
                    None => Err(format!(
                        "[ERROR]: missing value for argument {short} {long}"
                    )),
                    Some(value) => Ok(value.clone()),
                };
            }
        }

        Err(format!("[ERROR]: missing argument {short} {long}"))
    }

    pub fn string_list(&self, short: &str, long: &str) -> Result<Vec<String>, String> {
        let mut values = vec![];
        for mut i in 0..self.args.len() {
            if self.args[i] == short || self.args[i] == long {
                i += 1;

                while i < self.args.len() && !self.args[i].starts_with("-") {
                    values.push(self.args[i].clone());
                    i += 1;
                }
            }
        }

        match values.is_empty() {
            true => Err(format!(
                "[ERROR]: missing values for argument {short} {long}"
            )),
            false => Ok(values),
        }
    }
}

pub fn deindent(content: &str) -> String {
    let intent = content.len() - content.trim_start().len() - 1;

    let mut output = String::new();
    for line in content.trim_end().lines() {
        if line.trim().is_empty() {
            output.push('\n');
            continue;
        }

        output.push_str(line.get(intent..).unwrap_or("\n"));
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_args() {
        let mut builder = ArgBuilder::new(vec!["--help".to_string()]);
        assert!(builder.bool("--help", "-h"));

        builder = ArgBuilder::new(vec!["-h".to_string()]);
        assert!(builder.bool("--help", "-h"));

        builder = ArgBuilder::new(vec!["-h".to_string()]);
        assert!(!builder.bool("--help", "-H"));
    }

    #[test]
    fn string_args() {
        let mut builder = ArgBuilder::new(vec!["--help".to_string(), "value".to_string()]);

        assert_eq!(Ok("value".into()), builder.string("--help", "-h"));

        builder = ArgBuilder::new(vec!["-h".to_string(), "value".to_string()]);
        assert_eq!(Ok("value".into()), builder.string("--help", "-h"));
        assert_eq!(
            Err("[ERROR]: missing argument --no -n".into()),
            builder.string("--no", "-n")
        );
    }

    #[test]
    fn string_list() {
        let args = vec!["--test", "value1", "value2", "-h", "--test", "value3"];
        let builder = ArgBuilder::new(args.into_iter().map(|s| s.to_string()).collect());

        let expected = vec!["value1", "value2", "value3"];
        assert_eq!(
            Ok(expected.into_iter().map(|s| s.to_string()).collect()),
            builder.string_list("--test", "-t")
        );
    }
}
