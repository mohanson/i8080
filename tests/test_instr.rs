use i8080::Flag;

mod help;

#[test]
fn test_daa() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.mem.set(0x0000, 0x27);
    cpu.reg.a = 0x9b;
    cpu.next();
    assert_eq!(cpu.reg.a, 1);
    assert!(cpu.reg.get_flag(Flag::A));
    assert!(cpu.reg.get_flag(Flag::C));
}
