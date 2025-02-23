use crate::UciResult;

#[derive(Debug)]
pub struct UciOptionType {
    pub name: String,
    pub data_type: String,
    pub default: String,
}

impl UciOptionType {
    pub fn from_vec(parts: Vec<&str>) -> UciResult<UciOptionType> {
        let name_start = 2;
        let name_end = parts
            .iter()
            .position(|&p| p == "type")
            .unwrap_or(parts.len());
        let name = parts[name_start..name_end].join(" ");

        let type_start = parts.iter().position(|&p| p == "type").unwrap_or(0) + 1;
        let data_type = parts
            .get(type_start)
            .cloned()
            .unwrap_or("unknown")
            .to_string();

        let default_start = parts.iter().position(|&p| p == "default").map(|i| i + 1);
        let default = default_start
            .and_then(|start| parts.get(start).cloned())
            .unwrap_or("<empty>")
            .to_string();

        Ok(UciOptionType {
            name,
            data_type,
            default,
        })
    }
}

#[derive(Debug, Default)]
pub struct UciCommandResult {
    pub name: String,
    pub author: String,
    pub options: Vec<UciOptionType>,
}

impl UciCommandResult {
    pub fn from_str(s: &str) -> UciResult<UciCommandResult> {
        let mut result = UciCommandResult::default();

        for line in s.lines() {
            let items: Vec<&str> = line.split_whitespace().collect();
            match items.first() {
                Some(&"id") => {
                    if items.get(1) == Some(&"name") {
                        result.name = items[2..].join(" ");
                    } else if items.get(1) == Some(&"author") {
                        result.author = items[2..].join(" ");
                    }
                }
                Some(&"option") => {
                    result.options.push(UciOptionType::from_vec(items)?);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}
