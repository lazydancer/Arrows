run:
	@echo "Running"
	cd src/bridge_rust_side && cargo build --release
	cd src && python3 -m draft 
