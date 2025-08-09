use std::collections::HashMap;
use crate::parsing::ast::Identifier;

#[derive(Debug, Clone)]
pub enum Doc {
    Desc(String),
    FnDesc { desc: String, params: HashMap<Identifier, String>, returns: String },
}

impl Doc {
    pub fn parse(comment: &str) -> Option<Doc> {
        if !comment.trim_start().starts_with("/**") {
            return None;
        }
        
        let content = comment
            .trim_start_matches("/**")
            .trim_end_matches("*/")
            .lines()
            .map(|line| {
                line.trim_start()
                    .trim_start_matches('*')
                    .trim_start()
                    .to_string()
            })
            .collect::<Vec<_>>();

        let mut desc_lines = Vec::new();
        let mut params = HashMap::new();
        let mut returns_lines = Vec::new();

        enum Mode {
            Desc,
            Param(Identifier),
            Returns,
        }

        let mut mode = Mode::Desc;

        for line in content {
            if line.starts_with("@param") {
                let mut parts = line["@param".len()..].trim().splitn(2, ' ');
                if let Some(name) = parts.next() {
                    let desc = parts.next().unwrap_or("").to_string();
                    let id = Identifier(name.to_string());
                    params.insert(id.clone(), vec![desc]);
                    mode = Mode::Param(id);
                } else {
                    mode = Mode::Desc;
                }
            } else if line.starts_with("@returns") {
                let desc = line["@returns".len()..].trim().to_string();
                returns_lines.clear();
                returns_lines.push(desc);
                mode = Mode::Returns;
            } else if line.starts_with('@') {
                mode = Mode::Desc;
                desc_lines.push(line);
            } else {
                match &mut mode {
                    Mode::Desc => desc_lines.push(line),
                    Mode::Param(id) => {
                        if let Some(v) = params.get_mut(id) {
                            v.push(line);
                        }
                    }
                    Mode::Returns => returns_lines.push(line),
                }
            }
        }

        let desc_str = desc_lines.join("\n").trim().to_string();
        let params_str = params
            .into_iter()
            .map(|(k, v)| (k, v.join("\n").trim().to_string()))
            .collect::<HashMap<_, _>>();
        let returns_str = returns_lines.join("\n").trim().to_string();

        if !params_str.is_empty() || !returns_str.is_empty() {
            Some(Doc::FnDesc {
                desc: desc_str,
                params: params_str,
                returns: returns_str,
            })
        } else {
            Some(Doc::Desc(desc_str))
        }
    }
}