all: android windows linux
android:
	cargo build --lib --release --target aarch64-unknown-linux-gnu
windows:
	cargo build --lib --release --target x86_64-pc-windows-gnu
linux:
	cargo build --lib --release --target x86_64-unknown-linux-gnu