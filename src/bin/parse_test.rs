use std::io;
use chipper8::parser;

fn main() -> io::Result<()> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let instruction = parser::parse_instruction(line.as_str());
        println!("{:?}", instruction);
    }
}