CC       = rustc

LIB_SRC  = src/lib.rs

OUT_DIR  = lib

.PHONY: build
build:
	@mkdir -p $(OUT_DIR)
	$(CC) --crate-type=rlib $(LIB_SRC) --out-dir $(OUT_DIR)
