pub struct TokenType;

#[allow(dead_code)]
impl TokenType {
    pub const NAMESPACE: u32 = 0;
    pub const TYPE: u32 = 1;
    pub const CLASS: u32 = 2;
    pub const ENUM: u32 = 3;
    pub const INTERFACE: u32 = 4;
    pub const STRUCT: u32 = 5;
    pub const TYPE_PARAMETER: u32 = 6;
    pub const PARAMETER: u32 = 7;
    pub const VARIABLE: u32 = 8;
    pub const PROPERTY: u32 = 9;
    pub const ENUM_MEMBER: u32 = 10;
    pub const EVENT: u32 = 11;
    pub const FUNCTION: u32 = 12;
    pub const METHOD: u32 = 13;
    pub const MACRO: u32 = 14;
    pub const KEYWORD: u32 = 15;
    pub const MODIFIER: u32 = 16;
    pub const COMMENT: u32 = 17;
    pub const STRING: u32 = 18;
    pub const NUMBER: u32 = 19;
    pub const REGEXP: u32 = 20;
    pub const OPERATOR: u32 = 21;
    pub const DECORATOR: u32 = 22;
}
