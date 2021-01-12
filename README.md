# Circuit Dojo nRf9160 Feather Tester Crate

In this project, i'm using `pyocd` to run the GDB server. Install on Python 3.7 using `pip3 install pyocd`.

You also need to install the board definitions like so:

```
pyocd pack -f atsamd21
 Part            Vendor   Pack         Version   Installed
-------------------------------------------------------------
  ATSAMD21E15A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E15B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E15BU   Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E15L    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E16A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E16B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E16BU   Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E16L    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E17A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21E18A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G15A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G15B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G15L    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G16A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G16B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G16L    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G17A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G17AU   Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G18A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21G18AU   Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J15A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J15B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J16A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J16B    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J17A    Keil     SAMD21_DFP   1.3.0     True
  ATSAMD21J18A    Keil     SAMD21_DFP   1.3.0     True
```

This searches for the board pack that you need to work with `pyocd`

Then run

```
pyocd pack -i ATSAMD21J16B
```

Then you'll be set!

(*Also* this project requires Rust 1.42 or greater. [Download here.](https://www.rust-lang.org/learn/get-started))

## Programming

**The long way**

```
pyocd flash -t ATSAMD21J16B -f 4000000 target/thumbv6m-none-eabi/debug/examples/blinky_basic --format elf
```

**The short way**

```
make flash
```

## Debugging

In one terminal run:

```
$ make debug-server
```

In the second run

```
$ make debug-client
```

Using the `gdb` client, run in the console:

```
target remote :3333
```

To start the execution, type `c` in the gdb menu:

```
$ make debug-client
/Users/jaredwolff/gcc-arm-none-eabi-9-2019-q4-major/bin/arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/examples/blinky_basic
GNU gdb (GNU Tools for Arm Embedded Processors 9-2019-q4-major) 8.3.0.20190709-git
Copyright (C) 2019 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "--host=x86_64-apple-darwin10 --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/thumbv6m-none-eabi/debug/examples/blinky_basic...
cortex_m_rt::HardFault_ (ef=0x20001da8)
    at /Users/jaredwolff/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs:552
552         loop {
Breakpoint 1 at 0x4570: file /Users/jaredwolff/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs, line 562.
--Type <RET> for more, q to quit, c to continue without paging--
Breakpoint 2 at 0x46f4: file /Users/jaredwolff/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs, line 552.
Breakpoint 3 at 0x428c: file /Users/jaredwolff/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-semihosting-0.5.3/src/lib.rs, line 78.
Loading section .vector_table, size 0xb0 lma 0x0
Loading section .text, size 0x4650 lma 0xb0
Loading section .rodata, size 0x924 lma 0x4700
Start address 0x4530, load size 20516
Transfer rate: 2 KB/sec, 1709 bytes/write.
(gdb) c
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.
```

Use the `debug-output` target to view the output

```
$ make debug-output
```

**Note:** `make debug-client` and `make debug-server` are both needed to run the example with semihosting. (i.e. debug console over SWD)
Removing the semi-hosting print commands will allow normal exeuction.

## Production mode
To use *without* semi-hosting, simply re

## Testing

- [x] Testing power supplies
  - [x] VBAT
  - [x] 5V USB
- [x] GPIO Pins tested
  - [x] En
  - [x] Reset
  - [x] D12-D0
  - [x] A0-A5
- [x] Analog measurements
  - [x] Voltages
  - [ ] Current