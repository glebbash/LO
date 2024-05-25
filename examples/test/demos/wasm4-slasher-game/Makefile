OUTPUT := build/cart.wasm

$(OUTPUT): $(wildcard src/*)
	mkdir -p build
	wasmtime --dir=. lo.wasm src/main.lo > $(OUTPUT)

.PHONY: all
all: $(OUTPUT)

.PHONY: clean
clean:
	rm -f $(OUTPUT)
