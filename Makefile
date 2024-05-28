SRC_DIR := $(shell "pwd")
TARGET_DIR := $(shell "pwd")/target

PROJECT_VERS := $(shell "$(SRC_DIR)/util/version.sh" --short)
PROJECT_VERS_NO_COMMITN := $(shell "$(SRC_DIR)/util/version.sh" --short --no-commitn) # necessary for extension packaging

CARGO := cargo
CARGOFLAGS :=
CARGO_TOML := $(SRC_DIR)/Cargo.toml

NPM := npm
NODE := node
VSCE := vsce

VSCE_SUBCOMMAND := package
VSCE_FLAGS := --no-update-package-json --no-git-tag-version
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

# use EXT_PUBLISH=1 to publish extensions to relevant marketplaces
ifeq "$(EXT_PUBLISH)" "1"
VSCE_SUBCOMMAND = publish
else
VSCE_FLAGS += --out=$(VSCODE_EXT_DIST_DIR)
endif

.PHONY: devinit vscode_ext
.PHONY: clean


#
# All targets
#
all: devinit vscode_ext


#
# Compile the devinit executable
#

devinit: $(CARGO_TOML) | validate_cargo
	DEVINITVERS=$(PROJECT_VERS) \
	$(CARGO) build $(CARGOFLAGS) --manifest-path=$(CARGO_TOML)

#
# Bundle the VS Code extension
#

$(VSCODE_EXT_PREFIX)/node_modules:
	$(NPM) install --prefix=$(VSCODE_EXT_PREFIX)

vscode_ext: | validate_npm $(VSCODE_EXT_PREFIX)/node_modules
	$(NPM) run $(VSCODE_EXT_NPM_SCRIPT) --prefix=$(VSCODE_EXT_PREFIX)

ifneq "$(DEBUG)" "1"
	cp $(SRC_DIR)/LICENCE $(VSCODE_EXT_PREFIX)/LICENSE
	cp $(SRC_DIR)/resources/icon.png $(VSCODE_EXT_DIST_DIR)/icon.png

	cd $(VSCODE_EXT_PREFIX) && \
	$(VSCE) $(VSCE_SUBCOMMAND) $(VSCE_FLAGS) $(subst v,,$(PROJECT_VERS_NO_COMMITN))
endif


#
# Remove build directory
#

clean:
	rm -rf $(TARGET_DIR)
	rm -rf $(VSCODE_EXT_DIST_DIR)
	rm -rf $(VSCODE_EXT_PREFIX)/LICENSE
