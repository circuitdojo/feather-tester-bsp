TOOLCHAIN_BIN_DIR := /Users/jaredwolff/gcc-arm-none-eabi-9-2019-q4-major/bin
TARGET := target/thumbv6m-none-eabi/debug/examples/main
MCU_TARGET := atsamd21j16b
DEBUG := false

# Disabling debug allows to run the code without debugger attached.
ifeq ($(DEBUG), true)
FEATURES := --features use_semihosting
else
FEATURES :=
endif

default: build

.PHONY: build
build:
	@cargo build --example main $(FEATURES)
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
