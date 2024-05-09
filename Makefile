BUILD_DIR := build
SRC_DIR := .

# PROJECT_VERS := $(shell "$(SRC_DIR)/util/version.sh" --short)
# PROJECT_VERS_LONG := $(shell "$(SRC_DIR)/util/version.sh" --long)

CARGO := cargo
CARGOCHAN := +nightly
CARGOFLAGS := -Zunstable-options
CARGO_TOML := $(SRC_DIR)/Cargo.toml

# this ensures `all` is run by default despite not being the first target in the Makefile
.DEFAULT_GOAL := all

# check for dependencies

.PHONY: validate_cargo
validate_cargo:
	$(if \
		$(shell which $(CARGO)),\
		$(info Cargo located at $(shell command -v $(CARGO))),\
		$(error Cargo not found in PATH, but is required to build devinit))
	@:

# run with DEBUG=1 to use debug configuration

ifneq "$(DEBUG)" "1"
CARGOFLAGS += --release
endif

.PHONY: devinit
.PHONY: clean


#
# All targets
#
all: devinit


$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)


#
# Compile the devinit executable
#

devinit: $(CARGO_TOML) | validate_cargo
	$(CARGO) $(CARGOCHAN) build $(CARGOFLAGS) --manifest-path=$(CARGO_TOML) --target-dir=$(BUILD_DIR)/_$@ --out-dir=$(BUILD_DIR)


#
# Remove build directory
#

clean:
	rm -r $(BUILD_DIR)
