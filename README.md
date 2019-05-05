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
Test loaded: "./res/cpu_tests/TST8080.COM"
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC
 VERSION 1.0  (C) 1980

 CPU IS OPERATIONAL

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
Test loaded: "./res/cpu_tests/8080EXM.COM"
8080 instruction exerciser
dad <b,d,h,sp>................  PASS! crc is:14474ba6
aluop nn......................  PASS! crc is:9e922f9e
aluop <b,c,d,e,h,l,m,a>.......  PASS! crc is:cf762c86
<daa,cma,stc,cmc>.............  PASS! crc is:bb3f030c
<inr,dcr> a...................  PASS! crc is:adb6460e
<inr,dcr> b...................  PASS! crc is:83ed1345
<inx,dcx> b...................  PASS! crc is:f79287cd
<inr,dcr> c...................  PASS! crc is:e5f6721b
<inr,dcr> d...................  PASS! crc is:15b5579a
<inx,dcx> d...................  PASS! crc is:7f4e2501
<inr,dcr> e...................  PASS! crc is:cf2ab396
<inr,dcr> h...................  PASS! crc is:12b2952c
<inx,dcx> h...................  PASS! crc is:9f2b23c0
<inr,dcr> l...................  PASS! crc is:ff57d356
<inr,dcr> m...................  PASS! crc is:92e963bd
<inx,dcx> sp..................  PASS! crc is:d5702fab
lhld nnnn.....................  PASS! crc is:a9c3d5cb
shld nnnn.....................  PASS! crc is:e8864f26
lxi <b,d,h,sp>,nnnn...........  PASS! crc is:fcf46e12
ldax <b,d>....................  PASS! crc is:2b821d5f
mvi <b,c,d,e,h,l,m,a>,nn......  PASS! crc is:eaa72044
mov <bcdehla>,<bcdehla>.......  PASS! crc is:10b58cee
sta nnnn / lda nnnn...........  PASS! crc is:ed57af72
<rlc,rrc,ral,rar>.............  PASS! crc is:e0d89235
stax <b,d>....................  PASS! crc is:2b0471e9
Tests complete
```

Tested in the following environments:

- Rust 1.34.0

# Licences

WTFPL
