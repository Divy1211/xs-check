pub struct TokenModifier;

#[allow(dead_code)]
impl TokenModifier {
    pub const NONE: u32 = 0;
    pub const DECLARATION: u32 = 2_u32.pow(0);
    pub const DEFINITION: u32 = 2_u32.pow(1);
    pub const READONLY: u32 = 2_u32.pow(2);
    pub const STATIC: u32 = 2_u32.pow(3);
    pub const DEPRECATED: u32 = 2_u32.pow(4);
    pub const ABSTRACT: u32 = 2_u32.pow(5);
    pub const ASYNC: u32 = 2_u32.pow(6);
    pub const MODIFICATION: u32 = 2_u32.pow(7);
    pub const DOCUMENTATION: u32 = 2_u32.pow(8);
    pub const DEFAULT_LIBRARY: u32 = 2_u32.pow(9);
}
