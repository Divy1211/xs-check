use std::collections::{HashMap, HashSet};
use crate::parsing::ast::{Identifier, Type};
use crate::r#static::info::{IdInfo};
use crate::utils::warnings_from_str;

#[derive(Debug, Clone)]
pub enum Doc {
    None,
    Ignore(HashSet<u32>),
    Desc(String),
    FnDesc {
        desc: String,
        params: HashMap<Identifier, (usize, String)>,
        returns: Option<String>,
        deprecated: Option<String>,
        nodiscard: bool,
        no_num_promo: bool,
    },
}

impl Doc {
    pub fn is_no_num_promo(&self) -> bool {
        !matches!(self, Doc::FnDesc { no_num_promo: false, .. })
    }

    pub fn deprecation_reason(&self) -> Option<&str> {
        match self {
            Doc::FnDesc { deprecated, .. } => deprecated.as_deref(),
            _ => None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Doc::None)
    }

    pub fn is_nodiscard(&self) -> bool {
        !matches!(self, Doc::FnDesc { nodiscard: false, .. })
    }

    pub fn parse(comment: &str) -> Result<Doc, &str> {
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
        let mut deprecated_lines = Vec::new();

        enum Mode {
            Desc,
            Param(Identifier),
            Returns,
            Deprecated,
        }

        let mut mode = Mode::Desc;
        let mut idx = 0usize;
        let mut nodiscard = true;
        let mut no_num_promo = true;

        for line in content {
            if let Some(line) = line.strip_prefix("@param") {
                let mut parts = line.trim().splitn(2, ' ');
                if let Some(name) = parts.next() {
                    let desc = parts.next().unwrap_or("").to_string();
                    let id = Identifier(name.to_string());
                    param_lines.insert(id.clone(), (vec![desc], idx));
                    mode = Mode::Param(id);
                    idx += 1;
                } else {
                    mode = Mode::Desc;
                }
            } else if let Some(line) = line.strip_prefix("@returns") {
                let desc = line.trim().to_string();
                return_lines.clear();
                return_lines.push(desc);
                mode = Mode::Returns;
            } else if let Some(line) = line.strip_prefix("@deprecated") {
                let desc = line.trim().to_string();
                deprecated_lines.clear();
                deprecated_lines.push(desc);
                mode = Mode::Deprecated;
            } else if line.starts_with("@allow_discard") {
                nodiscard = false;
            } else if line.starts_with("@allow_no_num_promo") {
                no_num_promo = false;
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
                    Mode::Deprecated => deprecated_lines.push(line),
                }
            }
        }

        let desc = desc_lines.join("\n").trim().to_string();
        let params = param_lines
            .into_iter()
            .map(|(k, (v, i))| (k, (i, v.join("\n").trim().to_string())))
            .collect::<HashMap<_, _>>();
        let returns = if return_lines.is_empty() {
            None
        } else {
            Some(return_lines.join("\n").trim().to_string())
        };
        let deprecated = if deprecated_lines.is_empty() {
            None
        } else {
            Some(deprecated_lines.join("\n").trim().to_string())
        };

        if !params.is_empty() || returns.is_some() || deprecated.is_some() || !nodiscard || !no_num_promo {
            Ok(Doc::FnDesc { desc, params, returns, nodiscard, no_num_promo, deprecated })
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
                if opts.is_empty() {
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
            Doc::FnDesc { desc, params, returns, deprecated, .. } => {
                let mut doc = format!(
                    "{}\n\n{}{}", sign,
                    deprecated.as_ref().map(|reason| format!("**Deprecated**:\n\n{reason}\n\n")).unwrap_or("".into()),
                    desc.clone()
                );
                if !params.is_empty() {
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
                if let Some(returns) = returns {
                    doc += &format!("\n\n**Returns**:\n\n{returns}");
                }
                doc
            }
        }
    }
}