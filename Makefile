install:
	cargo build
	sudo mv ./target/debug/lsm /usr/bin/

uninstall:
	sudo rm -rf /usr/bin/lsm