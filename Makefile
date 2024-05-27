BUILD_DIR := $(shell "pwd")/build
SRC_DIR := $(shell "pwd")

PROJECT_VERS := $(shell "$(SRC_DIR)/util/version.sh" --short)

CARGO := cargo
CARGOCHAN := +nightly
CARGOFLAGS := -Zunstable-options
CARGO_TOML := $(SRC_DIR)/Cargo.toml

NPM := npm
NODE := node
VSCE := vsce

VSCODE_EXT_PREFIX := $(SRC_DIR)/integration/vscode-ext
VSCODE_EXT_NPM_SCRIPT := build:dev
VSCODE_EXT_DIST_DIR := $(VSCODE_EXT_PREFIX)/dist

# this ensures `all` is run by default despite not being the first target in the Makefile
.DEFAULT_GOAL := all

# check for dependencies

.PHONY: validate_cargo

validate_cargo:
	$(if \
		$(shell which $(CARGO)),\
		$(info Cargo located at $(shell command -v $(CARGO))),\
		$(error Cargo not found in PATH, but is required to build devinit))

validate_npm:
	$(if \
		$(shell which $(NPM)),\
		$(info npm located at $(shell command -v $(NPM))),\
		$(error npm not found in PATH, but is required to build the devinit VS Code extension))
	$(if \
		$(shell which $(NODE)),\
		$(info Node.js located at $(shell command -v $(NODE))),\
		$(error Node.js not found in PATH, but is required to build the devinit VS Code extension))

validate_vsce:
	$(if \
		$(shell which $(VSCE)),\
		$(info vsce located at $(shell command -v $(VSCE))),\
		$(error vsce not found in PATH, but is required to build the devinit VS Code extension))

# run with DEBUG=1 to use debug configuration

ifneq "$(DEBUG)" "1"
CARGOFLAGS += --release
VSCODE_EXT_NPM_SCRIPT = build:prod

vscode_ext: | validate_vsce
endif

.PHONY: devinit vscode_ext
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
	DEVINITVERS=$(PROJECT_VERS) \
	$(CARGO) $(CARGOCHAN) build $(CARGOFLAGS) --manifest-path=$(CARGO_TOML) --target-dir=$(BUILD_DIR)/_$@ --out-dir=$(BUILD_DIR)

#
# Bundle the VS Code extension
#

$(VSCODE_EXT_PREFIX)/node_modules:
	$(NPM) install --prefix=$(VSCODE_EXT_PREFIX)

vscode_ext: | validate_npm $(VSCODE_EXT_PREFIX)/node_modules
	$(NPM) run $(VSCODE_EXT_NPM_SCRIPT) --prefix=$(VSCODE_EXT_PREFIX)

ifneq "$(DEBUG)" "1"
	cp $(SRC_DIR)/LICENCE $(VSCODE_EXT_PREFIX)/LICENSE

	cd $(VSCODE_EXT_PREFIX) && \
	$(VSCE) package --out=$(VSCODE_EXT_DIST_DIR) --readme-path=$(SRC_DIR)/README.md
endif


#
# Remove build directory
#

clean:
	rm -r $(BUILD_DIR)
