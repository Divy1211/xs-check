use ariadne::{Color, Fmt};

pub fn msg_fmt(mut msg: &str, keywords: &[String], color: &Color) -> String {
    let mut result = String::new();

    for (idx, kw) in keywords.iter().enumerate() {
        let idx = format!("{{{}}}", idx);
        if let Some(pos) = msg.find(&idx) {
            result.push_str(&msg[..pos]);
            result.push_str(&kw.fg(*color).to_string());
            msg = &msg[pos + idx.len()..];
        }
    }
    result.push_str(&msg);

    result
}