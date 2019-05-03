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

#[test]
fn test_rom_8080pre() {
    let mut mem = Box::new(help::Memory::new());
    load_test(&mut mem, "./res/cpu_tests/8080PRE.COM");
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.mem.set(0x0005, 0xc9);
    cpu.reg.pc = 0xf800;
    cpu.reg.pc = 0x0100;
    loop {
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
                        println!("");
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
            break;
        }
    }
}
