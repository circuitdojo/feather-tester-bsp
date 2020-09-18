TOOLCHAIN_BIN_DIR := /Users/jaredwolff/gcc-arm-none-eabi-9-2019-q4-major/bin
TARGET := target/thumbv6m-none-eabi/debug/examples/blinky_basic
MCU_TARGET := atsamd21j16b

default: build

.PHONY: build
build:
	@cargo build --example blinky_basic --features use_semihosting
	@$(TOOLCHAIN_BIN_DIR)/arm-none-eabi-objcopy -O binary \
    $(TARGET) \
    $(TARGET).bin

.PHONY: flash
flash: build
	pyocd flash $(TARGET).bin --format bin --target $(MCU_TARGET)

.PHONY: erase
erase:
	pyocd erase chip --target $(MCU_TARGET)

.PHONY: debug-server
debug-server: build
	pyocd gdb --semihosting --persist --target $(MCU_TARGET)

.PHONY: debug-output
debug-output:
	nc localhost 4444

.PHONY: debug-client
debug-client:
	$(TOOLCHAIN_BIN_DIR)/arm-none-eabi-gdb $(TARGET)
