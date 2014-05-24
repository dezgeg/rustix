SOURCES  := $(wildcard src/*.rs)
BINARIES := $(patsubst src/%.rs,out/%,$(SOURCES))

.PHONY: all
all: $(BINARIES)

.PHONY: clean
clean:
	rm -f $(BINARIES)

out/%: src/%.rs
	rustc $< -o $@
