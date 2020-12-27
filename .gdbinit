target extended-remote :3333

# Semihosting enable!
monitor arm semihosting enable

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

monitor halt
load
monitor reset