# Variables
CARGO := cargo
RUSTUP := rustup
NIGHTLY := nightly

# Reglas
.PHONY: all build run clean set-nightly

# Configura el entorno para usar nightly
set-nightly:
	$(RUSTUP) install $(NIGHTLY)
	$(RUSTUP) override set $(NIGHTLY)

# Compila el proyecto
build:g
	$(CARGO) build

# Ejecuta el proyecto
run: build
	$(CARGO) run

# Limpia los archivos generados
clean:
	$(CARGO) clean

# Compila y ejecuta el proyecto
all: set-nightly run
