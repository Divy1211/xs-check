use std::collections::{HashMap, HashSet};
use crate::parsing::ast::{Identifier, Type};
use crate::r#static::info::{IdInfo};
use crate::utils::warnings_from_str;

#[derive(Debug, Clone)]
pub enum Doc {
    None,
    Ignore(HashSet<u32>),
    Desc(String),
    FnDesc { desc: String, params: HashMap<Identifier, (usize, String)>, returns: String },
}

impl Doc {
    pub fn is_none(&self) -> bool {
        match self {
            Doc::None => true,
            _ => false,
        }
    }
    
    pub fn  parse(comment: &str) -> Result<Doc, &str> {
        let comment = comment.trim_start();
        if comment.starts_with("// xsc-ignore: ") {
            let comment = comment.trim_start_matches("// xsc-ignore: ");
            return Ok(Doc::Ignore(warnings_from_str(comment)?));
        }
        if !comment.starts_with("/**") {
            return Ok(Doc::None);
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
        let mut param_lines = HashMap::new();
        let mut return_lines = Vec::new();

        enum Mode {
            Desc,
            Param(Identifier),
            Returns,
        }

        let mut mode = Mode::Desc;
        let mut idx = 0usize;

        for line in content {
            if line.starts_with("@param") {
                let mut parts = line["@param".len()..].trim().splitn(2, ' ');
                if let Some(name) = parts.next() {
                    let desc = parts.next().unwrap_or("").to_string();
                    let id = Identifier(name.to_string());
                    param_lines.insert(id.clone(), (vec![desc], idx));
                    mode = Mode::Param(id);
                    idx += 1;
                } else {
                    mode = Mode::Desc;
                }
            } else if line.starts_with("@returns") {
                let desc = line["@returns".len()..].trim().to_string();
                return_lines.clear();
                return_lines.push(desc);
                mode = Mode::Returns;
            } else if line.starts_with('@') {
                mode = Mode::Desc;
                desc_lines.push(line);
            } else {
                match &mut mode {
                    Mode::Desc => desc_lines.push(line),
                    Mode::Param(id) => {
                        if let Some((v, _i)) = param_lines.get_mut(id) {
                            v.push(line);
                        }
                    }
                    Mode::Returns => return_lines.push(line),
                }
            }
        }

        let desc = desc_lines.join("\n").trim().to_string();
        let params = param_lines
            .into_iter()
            .map(|(k, (v, i))| (k, (i, v.join("\n").trim().to_string())))
            .collect::<HashMap<_, _>>();
        let returns = return_lines.join("\n").trim().to_string();

        if !params.is_empty() || !returns.is_empty() {
            Ok(Doc::FnDesc { desc, params, returns })
        } else {
            Ok(Doc::Desc(desc))
        }
    }
    
    pub fn render(&self, id: &Identifier, info: &IdInfo) -> String {
        let sign = 'sign: { match &info.type_ {
            Type::Int | Type::Float | Type::Bool | Type::Str | Type::Vec | Type::Label => {
                let Some(init) = &info.init else {
                    break 'sign format!("```xs\n{} {}\n```", info.type_, id.0);
                };
                format!(
                    "```xs\nconst {} {} = {}\n```",
                    info.type_,
                    id.0,
                    init.lit_str().expect("Non-literal value found in const init")
                )
            }
            Type::Rule => {
                let opts = info.modifiers.get_rule_opts().expect("Rule missing opts");
                if opts.len() == 0 {
                    break 'sign format!("```xs\nrule {}\n```", id.0);
                }
                format!(
                    "```xs\nrule {}\n    {}\n```",
                    id.0, opts.iter()
                        .map(|opt| opt.render())
                        .collect::<Vec<_>>()
                        .join("\n    ")
                )
            }
            Type::Fn { is_mutable, type_sign } => {
                let rtype = &type_sign.last().expect("Function missing return type").1;
                let mutable = if *is_mutable { "mutable " } else { "" };
                if type_sign.len() == 1 {
                    break 'sign format!("```xs\n{mutable}{rtype} {}()\n```", id.0)
                }
                format!(
                    "```xs\n{mutable}{rtype} {}(\n    {}\n)\n```",
                    id.0,
                    type_sign[..type_sign.len()-1].iter()
                        .map(|(id, type_)| format!("{type_} {id}"))
                        .collect::<Vec<_>>()
                        .join("\n    ")
                )
            }
            _ => { unreachable!("Internal Error Occurred"); }
        }};
        
        match self {
            Doc::None | Doc::Ignore(_) => sign,
            Doc::Desc(desc) => {
                format!("{}\n\n{}", sign, desc.clone())
            },
            Doc::FnDesc { desc, params, returns } => {
                let mut doc = format!("{}\n\n{}", sign, desc.clone());
                if params.len() > 0 {
                    let mut params = params.iter().collect::<Vec<_>>();
                    params.sort_by_key(|(_id, (idx, _desc))| idx);

                    doc += &format!(
                        "\n\n**Parameters**:\n\n{}",
                        params.iter()
                            .map(|(id, (idx, desc))| format!("{}. **`{}:`** {}", idx+1, id, desc))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
                if returns.len() > 0 {
                    doc += &format!("\n\n**Returns**:\n\n{returns}");
                }
                doc
            }
        }
    }
}