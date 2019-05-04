# i8080

i8080 is a emulator for Intel 8080 cpu.

- [8080 Programmers Manual](http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
- [8080 opcodes](http://pastraiser.com/cpu/i8080/i8080_opcodes.html)

```toml
[dependencies]
i8080 = { git = "https://github.com/mohanson/i8080" }
```

# Tests

The test roms (cpu_tests folder) are taken from [http://altairclone.com/downloads/cpu_tests/](http://altairclone.com/downloads/cpu_tests/).

```sh
$ python ./scripts/get_cpu_tests.py
$ cargo run --example test_roms
```

```text
*******************
Test loaded: "./res/cpu_tests/8080PRE.COM"
8080 Preliminary tests complete

*******************
Test loaded: "./res/cpu_tests/CPUTEST.COM"

DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS 8080/8085
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK

*******************
Test loaded: "./res/cpu_tests/TST8080.COM"
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC
 VERSION 1.0  (C) 1980

 CPU IS OPERATIONAL
```

Tested in the following environments:

- Rust 1.34.0

# Licences

WTFPL
