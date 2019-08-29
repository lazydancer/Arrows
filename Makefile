run:
	@echo "Running"
	cd src/rust_side && cargo build --release
	cd src && python -m draft 
