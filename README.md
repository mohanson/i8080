# i8080

i8080 is a emulator for Intel 8080 cpu.

- [8080 Programmers Manual](http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
- [8080 opcodes](http://pastraiser.com/cpu/i8080/i8080_opcodes.html)

```toml
[dependencies]
i8080 = { git = "https://github.com/mohanson/i8080" }
```

Tested on:
- rustc 1.34.0

# Tests

The test roms (cpu_tests folder) are taken from [http://altairclone.com/downloads/cpu_tests/](http://altairclone.com/downloads/cpu_tests/).

```sh
$ python ./scripts/get_cpu_tests.py
$ cargo tests
```

# Licences

WTFPL
