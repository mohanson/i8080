use std::fs::File;
use std::io::Read;
use std::path::Path;

mod help;

fn load_test(mem: &mut help::Memory, path: impl AsRef<Path>) {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    mem.data[0x0100..(buf.len() + 0x0100)].clone_from_slice(&buf[..]);
    println!("Test loaded: {:?} Bytes from {:?}", buf.len(), path.as_ref());
}

fn exec_test(path: impl AsRef<Path>) {
    let mut mem = Box::new(help::Memory::new());
    load_test(&mut mem, path);
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.mem.set(0x0005, 0xc9);
    cpu.reg.pc = 0x0100;
    loop {
        let prev_pc = cpu.reg.pc;
        cpu.next();
        if cpu.reg.pc == 0x76 {
            panic!("")
        }
        if cpu.reg.pc == 0x05 {
            if cpu.reg.c == 0x09 {
                let mut a = cpu.reg.get_de();
                loop {
                    let c = cpu.mem.get(a);
                    if c as char == '$' {
                        break;
                    } else {
                        a += 1;
                    }
                    print!("{}", c as char);
                }
            }
            if cpu.reg.c == 0x02 {
                print!("{}", cpu.reg.e as char);
            }
        }
        if cpu.reg.pc == 0x00 {
            println!("");
            println!("Jump to 0 from 0x{:04x}", prev_pc);
            break;
        }
    }
}

#[test]
fn test_rom_8080pre() {
    exec_test("./res/cpu_tests/8080PRE.COM");
}

#[test]
fn test_rom_tst8080() {
    exec_test("./res/cpu_tests/TST8080.COM");
}

#[test]
fn test_rom_cputest() {
    exec_test("./res/cpu_tests/CPUTEST.COM");
}
