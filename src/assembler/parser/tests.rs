use crate::assembler::Tokens;
use crate::machine::{Instruction, OpCode};

#[test]
fn round_trip_opcode_instruction_text_instruction_opcode() {
    let mut invalid_opcodes = 0usize;
    for code in 0..0xFFFFu16 {
        let opcode = OpCode(code.into());
        let opcode_string = format!("{}", opcode);
        if let Ok(instruction) = Instruction::try_from(opcode) {
            let text = format!("{}", instruction);
            eprintln!("{}", text);
            let parsed: Instruction = Tokens::from(text.as_str()).try_into().unwrap();
            eprintln!("{}", opcode_string);
            let parsed_from_opcode_string: Instruction = Tokens::from(opcode_string.as_str()).try_into().unwrap();
            assert_eq!(parsed, instruction);
            assert_eq!(parsed_from_opcode_string, instruction);
            assert_eq!(code, OpCode::try_from(&instruction).unwrap().0.0);
        } else {
            invalid_opcodes += 1;
        }
    }
    eprintln!("{}", invalid_opcodes);
    assert_eq!(invalid_opcodes, 22192);
}
