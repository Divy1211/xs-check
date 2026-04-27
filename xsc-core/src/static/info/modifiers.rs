use crate::parsing::ast::RuleOpt;

#[derive(Debug, Clone)]
pub enum Modifiers {
    Var { is_static: bool, is_const: bool, is_extern: bool, is_export: bool },
    Rule { rule_opts: Vec<RuleOpt> },
}

impl Modifiers {
    pub fn var_none() -> Self {
        Modifiers::Var { is_static: false, is_const: false, is_extern: false, is_export: false }
    }

    pub fn var(is_static: bool, is_const: bool, is_extern: bool, is_export: bool) -> Self {
        Modifiers::Var { is_static, is_const, is_extern, is_export }
    }

    pub fn set_is_const(&mut self, value: bool) {
        match self {
            Modifiers::Var { is_const, .. } => *is_const = value,
            _ => {},
        }
    }
    
    pub fn rule(rule_opts: Vec<RuleOpt>) -> Self {
        Modifiers::Rule { rule_opts }
    }
    
    pub fn is_const(&self) -> bool {
        match self {
            Modifiers::Var { is_const, .. } => *is_const,
            _ => false,
        }
    }

    pub fn is_extern(&self) -> bool {
        match self {
            Modifiers::Var { is_extern, is_export, .. } => *is_extern || *is_export,
            _ => false,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Modifiers::Var { is_static, .. } => *is_static,
            _ => false,
        }
    }
    
    pub fn get_rule_opts(&self) -> Option<&Vec<RuleOpt>> {
        match self {
            Modifiers::Rule { rule_opts } => Some(rule_opts),
            _ => None,
        }
    }
}