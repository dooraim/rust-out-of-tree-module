# custom build ubuntu with rust

fare riferimento al [build custom](./custom_build-ubuntu.md) per le informazioni generali su come fare il build di ubuntu

fare il clone del repository 
```bash
git clone --depth 1 https://github.com/Rust-for-Linux/linux.git -b rust linux-rust
cd linux-rust
git checkout 18b749148
```
Scaricare la defconfig per Ubuntu che supporta anche Rust, questa è [arm64_defconfig](./defconfig/arm64_defconfig)

Fare il source del env di Rust
```bash
rustup override set $(scripts/min-tool-version.sh rustc)
rustup component add rust-src
cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen-cli
```
Controllare che rust sia configurato correttamente con il comando seguente
```bash
make LLVM=1 rustavailable
```
Caricare poi la defconfig
```bash
make LLVM=1 arm64_defconfig
```
Per evitare alcuni errori durante il build, eseguire i comandi seguenti
```bash
scripts/config --disable SYSTEM_TRUSTED_KEYS
scripts/config --disable SYSTEM_REVOCATION_KEYS
scripts/config --set-str CONFIG_SYSTEM_TRUSTED_KEYS ""
scripts/config --set-str CONFIG_SYSTEM_REVOCATION_KEYS ""
```
Eseguire i comandi seguenti
```bash
fakeroot LLVM=1 make -j $(nproc)
sudo make LLVM=1 -j $(nproc) modules_install
sudo make LLVM=1 install
sudo reboot
```
Per caricare il nuovo kernel premere SHIFT+F10 e poi andare nei settings

Per caricare il modulo del kernel in Rust eseguire i comandi seguenti
```bash
cp /lib/modules/6.3.0+/kernel/samples/rust/rust_print.ko ~/.
sudo insmod rust_print.ko
sudo dmesg | tail -n 17
[  191.434758] rust_print: Rust printing macros sample (init)
[  191.436694] rust_print: Emergency message (level 0) without args
[  191.437062] rust_print: Alert message (level 1) without args
[  191.437403] rust_print: Critical message (level 2) without args
[  191.437760] rust_print: Error message (level 3) without args
[  191.438100] rust_print: Warning message (level 4) without args
[  191.438451] rust_print: Notice message (level 5) without args
[  191.438868] rust_print: Info message (level 6) without args
[  191.439204] rust_print: A line that is continued without args
[  191.439552] rust_print: Emergency message (level 0) with args
[  191.439897] rust_print: Alert message (level 1) with args
[  191.440221] rust_print: Critical message (level 2) with args
[  191.440750] rust_print: Error message (level 3) with args
[  191.441263] rust_print: Warning message (level 4) with args
[  191.441887] rust_print: Notice message (level 5) with args
[  191.442182] rust_print: Info message (level 6) with args
[  191.442946] rust_print: A line that is continued with args
```

Se invece voglio fare il build di un modulo out-of-tree bisogna eseguire i comandi seguenti. Bisogna ricordarsi prima di fare il build del kernel, e installare i nuovi artefatti con i comandi sopra
```bash
git clone https://github.com/Rust-for-Linux/rust-out-of-tree-module.git
cd rust-out-of-tree-module
make LLVM=1
sudo insmod rust_out_of_tree.ko
```
Si ottine alla fine
```bash
sudo dmesg | tail -n 1
[  724.331917] rust_print: Hello World from Rust!
```
