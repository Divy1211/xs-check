use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
pub fn log(message: &str) {
    let path = r"C:\Users\Divy\My Stuff\web dev\VSCE\aoe2xs\server\xsc-lsp.log";
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{}", message);
    }
}