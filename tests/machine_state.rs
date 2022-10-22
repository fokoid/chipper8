use std::fs;
use chipper8::{Emulator, EmulatorConfig, Machine};

fn test_state(name: &str) {
    let rom_path = format!("tests/roms/{}.rom", name);
    let mut emulator = Emulator::new(EmulatorConfig {
        rom_path: rom_path.into(),
        fps: 1000,
        dump_path: None,
    }).unwrap();
    emulator.run().unwrap();

    let expected_path = format!("tests/data/{}.json", name);
    let expected: Machine = serde_json::from_str(&fs::read_to_string(&expected_path).unwrap()).unwrap();
    assert_eq!(emulator.machine, expected);
}

#[test]
fn test_state_init() {
    test_state("exit");
}