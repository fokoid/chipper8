use super::*;

#[test]
fn round_trip_opcode_instruction_opcode() {
    let mut invalid_opcodes = 0usize;
    for opcode in 0..0xFFFFu16 {
        let opcode = OpCode(opcode);
        if let Ok(instruction) = opcode.as_instruction() {
            assert_eq!(opcode.0, OpCode::try_from(&instruction).unwrap().0);
        } else {
            invalid_opcodes += 1;
        }
    }
    assert_eq!(invalid_opcodes, 44974);
}

#[test]
fn round_trip_opcode_instruction_text_instruction_opcode() {
    let mut invalid_opcodes = 0usize;
    for opcode in 0..0xFFFFu16 {
        let opcode = OpCode(opcode);
        if let Ok(instruction) = opcode.as_instruction() {
            let text = format!("{}", instruction);
            let parsed = Instruction::parse(Tokens::from(text.as_str())).unwrap();
            assert_eq!(parsed, instruction);
            assert_eq!(opcode.0, OpCode::try_from(&instruction).unwrap().0);
        } else {
            invalid_opcodes += 1;
        }
    }
    eprintln!("{}", invalid_opcodes);
    assert_eq!(invalid_opcodes, 44974);
}