Following along with the tutorial [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann.

To set up from scratch:

```
> cargo install cargo-xbuild
> cargo install bootimage
> rustup component add rust-src
> rustup component add llvm-tools-preview
> sudo  apt-get install qemu-system-x86
> cargo xbuild
> cargo xrun
```