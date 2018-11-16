all: crate

crate: stm32-rs
	python3 stm32ral.py . stm32-rs/svd/stm32*.svd.patched stm32-rs/cortex_m/armv*.svd.patched
	python3 make_supported_devices.py stm32-rs/stm32_part_table.yaml supported_devices.md

stm32-rs:
	+make -C stm32-rs patch
	+make -C stm32-rs/cortex_m patch

clean:
	rm -rf src/peripherals src/stm32* src/cortex_m src/lib.rs

buildall:
	for device in stm32-rs/devices/*.yaml; do\
		echo $$device;\
		cargo check --features `basename $$device .yaml` || break 0;\
		cargo check --features `basename $$device .yaml`,nosync || break 0;\
		cargo check --features `basename $$device .yaml`,rt  || break 0;\
	done

doc:
	cargo doc --features doc --no-default-features

.PHONY: all crate stm32-rs clean buildall doc
