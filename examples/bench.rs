use funcfmt::{FormatMap, Render, ToFormatPieces};
use std::fmt::Write;

fn main() {
    let mut formatters: FormatMap<String> = FormatMap::new();
    let mut fmtstr = String::new();
    let mut expected = String::new();

    for i in 1..20 {
        formatters.insert(i.to_string().into(), |e| Some(format!("_{e}_")));
        write!(&mut fmtstr, "{{{}}}", i).unwrap();
        write!(&mut expected, "_bar_").unwrap();
    }

    for _ in 1..100000 {
        let fp = formatters.to_format_pieces(&fmtstr).unwrap();
        let inp = String::from("bar");
        let fmt = fp.render(&inp).unwrap();
        assert_eq!(fmt, expected);
    }
}