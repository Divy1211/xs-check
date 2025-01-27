use ariadne::ReportKind;

use crate::parsing::ast::Identifier;
use crate::parsing::span::Span;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
pub enum XSError {
    // type errors
    ExtraArg { fn_name: String, span: Span },
    TypeMismatch { actual: String, expected: String, span: Span, note: Option<String> },
    NotCallable { name: String, actual: String, span: Span },
    OpMismatch { op: String, type1: String, type2: String, span: Span, note: Option<String> },

    // name errors
    UndefinedName { name: String, span: Span },
    RedefinedName { name: String, span: Span, og_src_loc: SrcLoc, note: Option<String> },

    Syntax { span: Span, msg: String, keywords: Vec<String> },

    Warning { span: Span, msg: String, keywords: Vec<String>, kind: WarningKind },
}

#[derive(Debug, Clone)]
pub enum WarningKind {
    TopStrInit = 7,
    DupCase = 8,
    DiscardedFn = 9,
    BreakPt = 10,
    UnusableClasses = 11,
    FirstOprArith = 12,
    CmpSilentCrash = 13,
    BoolCaseSilentCrash = 14,
    NumDownCast = 15,
    NoNumPromo = 16,
}

impl XSError {
    pub fn extra_arg(fn_name: &str, span: &Span) -> XSError {
        XSError::ExtraArg {
            fn_name: String::from(fn_name),
            span: span.clone(),
        }
    }

    pub fn type_mismatch(actual: &str, expected: &str, span: &Span, note: Option<&str>) -> XSError {
        XSError::TypeMismatch {
            actual: String::from(actual),
            expected: String::from(expected),
            span: span.clone(),
            note: note.map(String::from),
        }
    }

    pub fn not_callable(name: &Identifier, actual: &str, span: &Span) -> XSError {
        XSError::NotCallable {
            name: String::from(&name.0),
            actual: String::from(actual),
            span: span.clone(),
        }
    }

    pub fn op_mismatch(op: &str, type1: &str, type2: &str, span: &Span, note: Option<&str>) -> XSError {
        XSError::OpMismatch {
            op: String::from(op),
            type1: String::from(type1),
            type2: String::from(type2),
            span: span.clone(),
            note: note.map(String::from),
        }
    }

    pub fn undefined_name(name: &Identifier, span: &Span) -> XSError {
        XSError::UndefinedName {
            name: String::from(&name.0),
            span: span.clone(),
        }
    }

    pub fn redefined_name(name: &Identifier, span: &Span, og_src_loc: &SrcLoc, note: Option<&str>) -> XSError {
        XSError::RedefinedName {
            name: String::from(&name.0),
            span: span.clone(),
            og_src_loc: og_src_loc.clone(),
            note: note.map(String::from),
        }
    }

    pub fn syntax(span: &Span, msg: &str, keywords: Vec<&str>) -> XSError {
        XSError::Syntax {
            span: span.clone(),
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
        }
    }

    pub fn warning(span: &Span, msg: &str, keywords: Vec<&str>, kind: WarningKind) -> XSError {
        XSError::Warning {
            span: span.clone(),
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
            kind,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            XSError::ExtraArg { span, .. } => { span }
            XSError::TypeMismatch { span, .. } => { span }
            XSError::NotCallable { span, .. } => { span }
            XSError::OpMismatch { span, .. } => { span }
            XSError::UndefinedName { span, .. } => { span }
            XSError::RedefinedName { span, .. } => { span }
            XSError::Syntax { span, .. } => { span }
            XSError::Warning { span, .. } => { span }
        }
    }

    pub fn report_kind(&self) -> ReportKind {
        match self {
            XSError::Warning { .. } => { ReportKind::Warning }
            _ => { ReportKind::Error }
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            XSError::ExtraArg { .. } => { "TypeError" }
            XSError::TypeMismatch { .. } => { "TypeError" }
            XSError::NotCallable { .. } => { "TypeError" }
            XSError::OpMismatch { .. } => { "TypeError" }

            XSError::UndefinedName { .. } => { "NameError" }
            XSError::RedefinedName { .. } => { "NameError" }

            XSError::Syntax { .. } => { "SyntaxError" }

            XSError::Warning { kind: type_, .. } => { type_.as_str() }
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            XSError::ExtraArg { .. } => { 0 }
            XSError::TypeMismatch { .. } => { 1 }
            XSError::NotCallable { .. } => { 2 }
            XSError::OpMismatch { .. } => { 3 }
            XSError::UndefinedName { .. } => { 4 }
            XSError::RedefinedName { .. } => { 5 }
            XSError::Syntax { .. } => { 6 }
            XSError::Warning { kind, .. } => { kind.as_u32() }
        }
    }
}

impl WarningKind {
    pub fn as_u32(&self) -> u32 {
        self.clone() as u32
    }

    pub fn as_str(&self) -> &str {
        match self {
            WarningKind::TopStrInit => { "TopStrInit" }
            WarningKind::DupCase => { "DupCase" }
            WarningKind::DiscardedFn => { "DiscardedFn" }
            WarningKind::BreakPt => { "BreakPt" }
            WarningKind::UnusableClasses => { "UnusableClasses" }
            WarningKind::FirstOprArith => { "FirstOprArith" }
            WarningKind::CmpSilentCrash => { "CmpSilentCrash" }
            WarningKind::BoolCaseSilentCrash => { "BoolCaseSilentCrash" }
            WarningKind::NumDownCast => { "NumDownCast" }
            WarningKind::NoNumPromo => { "NoNumPromo" }
        }
    }

    pub fn from_str(name: &str) -> Option<WarningKind> {
        match name {
            "TopStrInit" => { Some(WarningKind::TopStrInit) }
            "DupCase" => { Some(WarningKind::DupCase) }
            "DiscardedFn" => { Some(WarningKind::DiscardedFn) }
            "BreakPt" => { Some(WarningKind::BreakPt) }
            "UnusableClasses" => { Some(WarningKind::UnusableClasses) }
            "FirstOprArith" => { Some(WarningKind::FirstOprArith) }
            "CmpSilentCrash" => { Some(WarningKind::CmpSilentCrash) }
            "BoolCaseSilentCrash" => { Some(WarningKind::BoolCaseSilentCrash) }
            "NumDownCast" => { Some(WarningKind::NumDownCast) }
            "NoNumPromo" => { Some(WarningKind::NoNumPromo) }
            _ => None
        }
    }
}