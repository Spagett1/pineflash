PREFIX ?= /usr/local
CARGO ?= cargo

UNAME_S := $(shell uname)
UNAME_M := $(shell uname -m)

BLISP_MACOS_URL ?= "https://github.com/pine64/blisp/releases/download/v0.0.4/blisp-apple-x86_64-v0.0.4.zip"
BLISP_LINUX_URL ?= "https://github.com/pine64/blisp/releases/download/v0.0.4/blisp-linux-x86_64-v0.0.4.zip"
BLISP_OTHER_URL ?= "https://github.com/pine64/blisp.git"

BLISP_MACOS_PKG ?= "blisp-apple-x86_64-v0.0.4.zip"
BLISP_LINUX_PKG ?= "blisp-linux64-v0.0.4.zip"
BLISP_OTHER_PKG ?= "blisp-src"

.PHONY: all build install install-macos install-linux64 install-other blisp-macos blisp-linux64 blisp-other

all: build

build: check-cargo
	$(CARGO) build --release

install: build
ifeq ($(UNAME_S),Darwin)
	$(MAKE) install-macos
else ifeq ($(UNAME_M),x86_64)
	$(MAKE) install-linux64
else
	$(MAKE) install-other
endif

install-macos: blisp-macos
	install -D -m 0755 ./target/release/pineflash "$(PREFIX)/bin/pineflash"

install-linux64: blisp-linux64
	install -D -m 0644 ./assets/Pineflash.desktop "$(PREFIX)/share/applications/Pineflash.desktop"
	install -D -m 0644 ./assets/pine64logo.png "$(PREFIX)/share/pixmaps/pine64logo.png"
	install -D -m 0755 ./target/release/pineflash "$(PREFIX)/bin/pineflash"

install-other: blisp-other
	install -D -m 0644 ./assets/Pineflash.desktop "$(PREFIX)/share/applications/Pineflash.desktop"
	install -D -m 0644 ./assets/pine64logo.png "$(PREFIX)/share/pixmaps/pine64logo.png"
	install -D -m 0755 ./target/release/pineflash "$(PREFIX)/bin/pineflash"

blisp-macos:
	curl -L "$(BLISP_MACOS_URL)" -o "$(BLISP_MACOS_PKG)"
	unzip -o "$(BLISP_MACOS_PKG)"
	chmod +x ./blisp
	install -D -m 0755 ./blisp "$(PREFIX)/bin/blisp"

blisp-linux64:
	curl -L "$(BLISP_LINUX_URL)" -o "$(BLISP_LINUX_PKG)"
	unzip -o "$(BLISP_LINUX_PKG)"
	install -D -m 0755 ./blisp "$(PREFIX)/bin/blisp"

blisp-other:
	git clone --recursive "$(BLISP_OTHER_URL)" "$(BLISP_OTHER_PKG)"
	cd "$(BLISP_OTHER_PKG)" && mkdir -p build && cd build && cmake -DBLISP_BUILD_CLI=ON .. && cmake --build .
	install -D -m 0755 "./$(BLISP_OTHER_PKG)/build/tools/blisp/blisp" "$(PREFIX)/bin/blisp"
	rm -rf "$(BLISP_OTHER_PKG)"

check-cargo:
	@command -v "$(CARGO)" >/dev/null 2>&1 || (echo "cargo command is not installed. Cannot proceed. Please ensure cargo is installed and on the PATH"; exit 1)
