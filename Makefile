TOOLCHAIN_BIN_DIR := /opt/nordic/ncs/v1.4.1/toolchain/bin
EXAMPLE := usb_echo
TARGET := target/thumbv6m-none-eabi/debug/examples/$(EXAMPLE)
MCU_TARGET := atsamd21j18a
DEBUG := false

FEATURES := usb

# Disabling debug allows to run the code without debugger attached.
ifeq ($(DEBUG), true)
FEATURES += use_semihosting
endif

default: build

.PHONY: build
build:
	cargo build --example $(EXAMPLE) --features "$(FEATURES)"
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
