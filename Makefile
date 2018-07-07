crate: stm32-rs
	python3 stm32ral.py . stm32-rs/svd/stm32*.svd.patched
	python3 make_supported_devices.py stm32-rs/stm32_part_table.yaml supported_devices.md

stm32-rs:
	make -C stm32-rs patch

clean:
	rm -rf src/peripherals src/stm32* src/lib.rs

buildall:
	for device in stm32-rs/devices/*.yaml; do\
		echo $$device;\
		cargo build --features `basename $$device .yaml`;\
	done

.PHONY: stm32-rs
