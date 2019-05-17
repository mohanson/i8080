use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use i8080::{Cpu, Linear, Memory};

fn load_test(mem: Rc<RefCell<Linear>>, path: impl AsRef<Path>) {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    mem.borrow_mut().data[0x0100..(buf.len() + 0x0100)].clone_from_slice(&buf[..]);
    println!("Test loaded: {:?}", path.as_ref());
}

fn exec_test(path: impl AsRef<Path>) {
    println!("*******************");
    let mem = Rc::new(RefCell::new(Linear::new()));
    load_test(mem.clone(), path);
    let mut cpu = Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0005, 0xc9);
    // Because tests used the pseudo instruction ORG 0x0100
    cpu.reg.pc = 0x0100;
    loop {
        if cpu.halted {
            break;
        }
        cpu.next();
        if cpu.reg.pc == 0x05 {
            if cpu.reg.c == 0x09 {
                let mut a = cpu.reg.get_de();
                loop {
                    let c = mem.borrow().get(a);
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
            println!("");
            break;
        }
    }
}

fn main() {
    exec_test("./res/cpu_tests/8080PRE.COM");
    exec_test("./res/cpu_tests/TST8080.COM");
    exec_test("./res/cpu_tests/CPUTEST.COM");
    exec_test("./res/cpu_tests/8080EXM.COM");
}
