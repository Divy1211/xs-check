use ariadne::ReportKind;

use crate::parsing::ast::Identifier;
use crate::parsing::span::Span;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
pub enum XsError {
    // type errors
    ExtraArg { fn_name: String, span: Span },
    TypeMismatch { actual: String, expected: String, span: Span, note: Option<String> },
    NotCallable { name: String, actual: String, span: Span },
    OpMismatch { op: String, type1: String, type2: String, span: Span, note: Option<String> },

    // name errors
    UndefinedName { name: String, span: Span },
    RedefinedName { name: String, span: Span, og_src_loc: SrcLoc, note: Option<String> },

    UnresolvedInclude { inc_filename: String, span: Span },
    
    Syntax { span: Span, msg: String, keywords: Vec<String> },

    Warning { span: Span, msg: String, keywords: Vec<String>, kind: WarningKind, ignored: bool },
}

#[derive(Debug, Clone)]
pub enum WarningKind {
    TopStrInit = 100,
    DupCase = 101,
    DiscardedFn = 102,
    BreakPt = 103,
    UnusableClasses = 104,
    FirstOprArith = 105,
    CmpSilentCrash = 106,
    BoolCaseSilentCrash = 107,
    NumDownCast = 108,
    NoNumPromo = 109,

    UnknownWarningName = 1000,
}

impl XsError {
    pub fn extra_arg(fn_name: &str, span: &Span) -> XsError {
        XsError::ExtraArg {
            fn_name: String::from(fn_name),
            span: *span,
        }
    }

    pub fn type_mismatch(actual: &str, expected: &str, span: &Span, note: Option<&str>) -> XsError {
        XsError::TypeMismatch {
            actual: String::from(actual),
            expected: String::from(expected),
            span: *span,
            note: note.map(String::from),
        }
    }

    pub fn not_callable(name: &Identifier, actual: &str, span: &Span) -> XsError {
        XsError::NotCallable {
            name: String::from(&name.0),
            actual: String::from(actual),
            span: *span,
        }
    }

    pub fn op_mismatch(op: &str, type1: &str, type2: &str, span: &Span, note: Option<&str>) -> XsError {
        XsError::OpMismatch {
            op: String::from(op),
            type1: String::from(type1),
            type2: String::from(type2),
            span: *span,
            note: note.map(String::from),
        }
    }

    pub fn undefined_name(name: &Identifier, span: &Span) -> XsError {
        XsError::UndefinedName {
            name: String::from(&name.0),
            span: *span,
        }
    }

    pub fn redefined_name(name: &Identifier, span: &Span, og_src_loc: &SrcLoc, note: Option<&str>) -> XsError {
        XsError::RedefinedName {
            name: String::from(&name.0),
            span: *span,
            og_src_loc: og_src_loc.clone(),
            note: note.map(String::from),
        }
    }

    pub fn unresolved_include(inc_filename: &str, span: &Span) -> XsError {
        XsError::UnresolvedInclude {
            inc_filename: inc_filename.into(),
            span: *span,
        }
    }
    
    pub fn syntax(span: &Span, msg: &str, keywords: Vec<&str>) -> XsError {
        XsError::Syntax {
            span: *span,
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
        }
    }

    pub fn warning(span: &Span, msg: &str, keywords: Vec<&str>, kind: WarningKind) -> XsError {
        XsError::Warning {
            span: *span,
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
            kind,
            ignored: false,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            XsError::ExtraArg { span, .. } => { span }
            XsError::TypeMismatch { span, .. } => { span }
            XsError::NotCallable { span, .. } => { span }
            XsError::OpMismatch { span, .. } => { span }
            XsError::UndefinedName { span, .. } => { span }
            XsError::RedefinedName { span, .. } => { span }
            XsError::UnresolvedInclude { span, .. } => { span }
            XsError::Syntax { span, .. } => { span }
            XsError::Warning { span, .. } => { span }
        }
    }

    pub fn report_kind(&self) -> ReportKind<'_> {
        match self {
            XsError::Warning { .. } => { ReportKind::Warning }
            _ => { ReportKind::Error }
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            XsError::ExtraArg { .. } => { "TypeError" }
            XsError::TypeMismatch { .. } => { "TypeError" }
            XsError::NotCallable { .. } => { "TypeError" }
            XsError::OpMismatch { .. } => { "TypeError" }

            XsError::UndefinedName { .. } => { "NameError" }
            XsError::RedefinedName { .. } => { "NameError" }

            XsError::UnresolvedInclude { .. } => { "UnresolvedInclude" }
            
            XsError::Syntax { .. } => { "SyntaxError" }

            XsError::Warning { kind: type_, .. } => { type_.as_str() }
        }
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, XsError::Warning { .. })
    }
    
    pub fn is_ignored(&self) -> bool {
        match self {
            XsError::Warning { ignored, .. } => *ignored,
            _ => false,
        }
    }
    
    pub fn code(&self) -> u32 {
        match self {
            XsError::ExtraArg { .. } => { 0 }
            XsError::TypeMismatch { .. } => { 1 }
            XsError::NotCallable { .. } => { 2 }
            XsError::OpMismatch { .. } => { 3 }
            XsError::UndefinedName { .. } => { 4 }
            XsError::RedefinedName { .. } => { 5 }
            XsError::UnresolvedInclude { .. } => { 6 }
            XsError::Syntax { .. } => { 7 }
            XsError::Warning { kind, .. } => { kind.as_u32() }
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
            WarningKind::UnknownWarningName => { "UnknownWarningName" }
        }
    }

    pub fn from_name(name: &str) -> Option<WarningKind> {
        match name {
            "TopStrInit"          => { Some(WarningKind::TopStrInit) }
            "DupCase"             => { Some(WarningKind::DupCase) }
            "DiscardedFn"         => { Some(WarningKind::DiscardedFn) }
            "BreakPt"             => { Some(WarningKind::BreakPt) }
            "UnusableClasses"     => { Some(WarningKind::UnusableClasses) }
            "FirstOprArith"       => { Some(WarningKind::FirstOprArith) }
            "CmpSilentCrash"      => { Some(WarningKind::CmpSilentCrash) }
            "BoolCaseSilentCrash" => { Some(WarningKind::BoolCaseSilentCrash) }
            "NumDownCast"         => { Some(WarningKind::NumDownCast) }
            "NoNumPromo"          => { Some(WarningKind::NoNumPromo) }
            
            // UnknownWarningName cannot be ignored, so it is exlcuded here
            _                     => None
        }
    }
}