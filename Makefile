.PHONY: default all

release_raspi: build_raspi deploy_raspi

deploy_raspi:
	scp ./target/aarch64-unknown-linux-gnu/release/aht20drv dima@10.42.0.1:/opt/aht20drv

build_raspi:
	cargo build --release --target=aarch64-unknown-linux-gnu

