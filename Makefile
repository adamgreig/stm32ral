crate: stm32-rs
	python3 stm32ral.py . stm32-rs/svd/stm32*.svd.patched

stm32-rs:
	make -C stm32-rs patch

clean:
	rm -rf src/peripherals src/stm32* src/lib.rs

.PHONY: stm32-rs
