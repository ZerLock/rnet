CC       = rustc

LIB_SRC  = src/lib.rs

OUT_DIR  = lib

.PHONY: build_lib
build_lib:
	@mkdir -p $(OUT_DIR)
	$(CC) --crate-type=rlib $(LIB_SRC) --out-dir $(OUT_DIR)
