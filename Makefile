# Variables
TARGET_DIR    := hulk
CARGO_DIR     := Compiler
OUT_LL        := $(CARGO_DIR)/out.ll
OUTPUT_TXT    := $(CARGO_DIR)/ast.txt
TARGETMac      := $(CARGO_DIR)/output_macos
TARGETWindows      := $(CARGO_DIR)/output.exe
TARGETLinux      := $(CARGO_DIR)/output_linux


.PHONY: compile execute clean

compile: $(TARGET_DIR)
	cd $(CARGO_DIR) && cargo run
	@if [ -f $(OUT_LL) ]; then mv $(OUT_LL) $(TARGET_DIR)/; fi
	@if [ -f $(OUTPUT_TXT) ]; then mv $(OUTPUT_TXT) $(TARGET_DIR)/; fi

$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

execute: compile
	@if [ -f $(TARGETMac) ]; then mv $(TARGETMac) $(TARGET_DIR)/; fi
	@if [ -f $(TARGETWindows) ]; then mv $(TARGETWindows) $(TARGET_DIR)/; fi
	@if [ -f $(TARGETLinux) ]; then mv $(TARGETLinux) $(TARGET_DIR)/; fi
	@if [ -f $(OUT_LL) ]; then mv $(OUT_LL) $(TARGET_DIR)/; fi
	@if [ -f $(OUTPUT_TXT) ]; then mv $(OUTPUT_TXT) $(TARGET_DIR)/; fi
clean:
	cd $(CARGO_DIR) && cargo clean
	rm -rf $(TARGET_DIR)