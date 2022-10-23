use super::*;

#[test]
fn round_trip_opcode_instruction_opcode() {
    let mut invalid_opcodes = 0usize;
    for code in 0..0xFFFFu16 {
        let opcode = OpCode(code.into());
        if let Ok(instruction) = Instruction::try_from(opcode) {
            assert_eq!(code, OpCode::try_from(&instruction).unwrap().0.0);
        } else {
            invalid_opcodes += 1;
        }
    }
    assert_eq!(invalid_opcodes, 44460);
}