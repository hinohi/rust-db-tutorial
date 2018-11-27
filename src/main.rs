use std::io::{stdin, stdout, BufRead, Write};

fn prompt<R, W>(reader: &mut R, writer: &mut W) -> String
where
    R: BufRead + ?Sized,
    W: Write,
{
    write!(writer, "db > ").unwrap();
    writer.flush().unwrap();
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

#[test]
fn test_prompt() {
    let read = b"select 1;";
    let mut reader = &read[..];
    let mut writer = Vec::new();
    let input = prompt(&mut reader, &mut writer);
    assert_eq!(String::from_utf8(writer).unwrap(), "db > ");
    assert_eq!(input, "select 1;")
}

fn main() {
    let stdin = stdin();
    let mut cin = stdin.lock();
    let mut cout = stdout();

    loop {
        let input = prompt(&mut cin, &mut cout);
        if input == ".exit" {
            return;
        } else {
            println!("Unrecognized command '{}'.", input);
        }
    }
}
