# Minimum kernel

https://os.phil-opp.com/ja/minimal-rust-kernel/

## 起動プロセス

1. マザーボードROMのファームウェアコード実行
2. RAMを検出。CPUとハードウェア初期化
3. Bootable Disk を探してOSの Kernel 起動

- **BIOS:** Basic Input/Output System. 古い。単純。
- **UEFI:** Unified Extensible Firmware Interface. 新しい。多機能。複雑

## BIOS の起動

- Bootloader: 先頭512バイトの実行可能コード
    - だいたいの Bootloader は512バイトより大きいので、1stステージと2ndステージに別れてる
- CPU のモードを以下の順番で変更してく
    - 16bit リアルモード
    - 32bit プロテクトモード
    - 64bit ロングモード
 - > ブートローダーを書くのにはアセンブリ言語を必要とするうえ、「何も考えずにプロセッサーのこのレジスタにこの値を書き込んでください」のような勉強の役に立たない作業がたくさんあるので、ちょっと面倒くさいです。
    - wwww
- bootimage: https://github.com/rust-osdev/bootimage

## Multiboot 標準規格

- ブートローダーの標準規格
- GNU GRUBが代表的

## Build Target

- ABI is 何: https://stackoverflow.com/questions/2171177/what-is-an-application-binary-interface-abi/2456882#2456882
    - Application Binary Interface らしい
3
## Disable Redzone

- https://os.phil-opp.com/red-zone/
- 割り込みを処理する場合、スタックポインタ最適化を無効化しないとダメらしい

## SIMD無効化

- 単一命令で複数のデータを扱う命令セット
- 色々複雑なので性能上問題になることも
- https://os.phil-opp.com/disable-simd/
- 浮動小数点演算ではSIMDレジスタを利用するため、SIMD無効化するとこれが使えない。これを解決するのが `soft-float` でエミュレートする

## カーネルビルド

```bash
cargo build --target x86_64-blog_os.json
   Compiling blog_os v0.1.0 (/Develop/rust/writing-os-rust)
warning: target json file contains unused fields: executable

error[E0463]: can't find crate for `core`
  |
  = note: the `x86_64-blog_os` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-blog_os`

error[E0463]: can't find crate for `compiler_builtins`

error[E0463]: can't find crate for `core`
 --> src/main.rs:4:5
```

めっちゃ怒られる。 `core` はコンパイル済みライブラリであるため、独自のターゲットには配布されていない。

### build-std

```toml
# in .cargo/config.toml

[unstable]
build-std = ["core", "compiler_builtins"]
```

cargo に core とかビルドし直す指定。nightly でしか使えないっぽい。

### Nightly　設定

```bash
rustup override set nightly
info: syncing channel updates for 'nightly-aarch64-apple-darwin'
info: latest update on 2023-07-25, rust version 1.73.0-nightly (31395ec38 2023-07-24)
info: downloading component 'cargo'
  5.5 MiB /   5.5 MiB (100 %)   1.0 MiB/s in  6s ETA:  0s
info: downloading component 'clippy'
  2.1 MiB /   2.1 MiB (100 %)   2.0 MiB/s in  2s ETA:  0s
info: downloading component 'rust-docs'
 13.7 MiB /  13.7 MiB (100 %)   1.5 MiB/s in 12s ETA:  0s
info: downloading component 'rust-std'
 24.2 MiB /  24.2 MiB (100 %)   1.4 MiB/s in 25s ETA:  0s 
info: downloading component 'rustc'
 52.1 MiB /  52.1 MiB (100 %)   1.2 MiB/s in 52s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.7 MiB /  13.7 MiB (100 %)   5.9 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 24.2 MiB /  24.2 MiB (100 %)  20.0 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 52.1 MiB /  52.1 MiB (100 %)  21.9 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'

  nightly-aarch64-apple-darwin installed - rustc 1.73.0-nightly (31395ec38 2023-07-24)

rustc --version
rustc 1.73.0-nightly (31395ec38 2023-07-24)
```

Nightlyで実行すると以下。

```bash
cargo build --target x86_64-blog_os.json
error: "$HOME/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/Cargo.lock" does not exist, unable to build with the standard library, try:
        rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
```

Toolchain　に対象がないとのこと。追加

```bash
rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
info: downloading component 'rust-src'
info: installing component 'rust-src'
```

これで再ビルド

```bash
cargo build --target x86_64-blog_os.json
  Updating crates.io index
  Downloaded hashbrown v0.14.0
  Downloaded getopts v0.2.21
  Downloaded allocator-api2 v0.2.15
  Downloaded compiler_builtins v0.1.95
  Downloaded 4 crates (369.9 KB) in 0.50s
   Compiling compiler_builtins v0.1.95
   Compiling core v0.0.0 (.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling blog_os v0.1.0 (/Develop/rust/writing-os-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 7.43s
```

独自ターゲットへコンパイルできた。

## メモリ関係の組み込み関数

- 組み込み関数の多くは `compiler_builtin` クレートによって提供される
- `memset` メモリブロック内の全てのバイトを与えられた値にセットする
- `memcpy` メモリブロックを他のブロックへコピーする
- `memcmp` 2つのメモリブロックを比較する

### 組み込み関数をコンパイラに与える

- 自前で `memset` 等を実装して `#[no_mangle]` Attributeを適用する
  - `#[no_mangle]` はコンパイル中の自動リネームを抑制する（一意になるようにコンパイラが勝手にリネームする
- あまりお勧めしない
- 未定義動作に繋がる
  - for ループを使って `memcpy` を実装すると無限再起呼び出しが発生しかねない
  - for ループは `IntoIterator::into_iter` トレイトメソッドを呼び出し、これが `memcpy` を呼び出す可能性があるから
- 既存のよくテストされた実装を使った方がBetter

### compiler_builtin クレート

- 必要な関数すべての実装が含まれてる
- 通常Cライブラリ実装と競合しないように無効化されてるだけ
  - ということは元々結構余計な実装がたくさん含まれているから全体的にでかいのか？
  - さすがに build 時には最適化されてると思うが
- `.cargo/config.toml` の `build-std-features` で有効化
  - `["compiler-builtins-mem"]`

### 標準ターゲット設定

- いちいち target 指定するのがかったるいのでデフォルトTarget設定
- `.cargo/config.toml` で以下を設定

```toml
[build]
target = "x86_64-blog_os.json"
```

## 画面出力

- VGAテキストバッファ
- バッファアドレス = `0xb8000`

```rust
static HELLO: &[u8] = b"Hello World!";

let vga_buffer = 0xb8000 as *mut u8;
for(i, &byte) in HELLO.iter().enumerate() {
    unsafe {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
}
```

- `0xb` はシアン色
- `unsafe` はVGAバッファで生アドレスを触っているため必要
- これで Build は通る

```bash
~/Develop/rust/writing-os-rust (main*) » cargo build
   Compiling compiler_builtins v0.1.95
   Compiling blog_os v0.1.0 (/Develop/rust/writing-os-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 2.62s
```

## カーネルを実行

- カーネルをブートローダーとリンクすることでブータブルティスクイメージにする必要がある
- QEMUとかで実行できる


### ブートイメージを作る

- `bootloader` クレートを使う
- `Cargo.toml` に依存を追加

```toml
[dependencies]
bootloader = "0.9.23"
```

- Build 後にブートローダーとリンクする必要がある
- しかし cargo は Build 後にスクリプトを走らせる機能がない
- `bootimage` を利用する

```bash
cargo install bootimage

    Updating crates.io index
  Downloaded bootimage v0.10.3
  Downloaded 1 crate (22.9 KB) in 0.94s
  Installing bootimage v0.10.3
    Updating crates.io index
  Downloaded thiserror-impl v1.0.44
  Downloaded thiserror v1.0.44
  Downloaded cargo_metadata v0.9.1
  Downloaded anyhow v1.0.72
  Downloaded semver-parser v0.7.0
  Downloaded semver v0.9.0
  Downloaded quote v1.0.32
  Downloaded wait-timeout v0.2.0
  Downloaded serde v1.0.176
  Downloaded syn v2.0.27
  Downloaded serde_derive v1.0.176
  Downloaded serde_json v1.0.104
  Downloaded json v0.12.4
  Downloaded locate-cargo-manifest v0.2.2
  Downloaded llvm-tools v0.1.1
  Downloaded 15 crates (1.1 MB) in 0.82s
   Compiling proc-macro2 v1.0.66
   Compiling unicode-ident v1.0.11
   Compiling serde v1.0.176
   Compiling serde_json v1.0.104
   Compiling libc v0.2.147
   Compiling anyhow v1.0.72
   Compiling itoa v1.0.9
   Compiling semver-parser v0.7.0
   Compiling thiserror v1.0.44
   Compiling ryu v1.0.15
   Compiling json v0.12.4
   Compiling llvm-tools v0.1.1
   Compiling locate-cargo-manifest v0.2.2
   Compiling quote v1.0.32
   Compiling syn v2.0.27
   Compiling wait-timeout v0.2.0
   Compiling thiserror-impl v1.0.44
   Compiling serde_derive v1.0.176
   Compiling semver v0.9.0
   Compiling toml v0.5.11
   Compiling cargo_metadata v0.9.1
   Compiling bootimage v0.10.3
    Finished release [optimized] target(s) in 10.18s
  Installing /.cargo/bin/bootimage
  Installing /.cargo/bin/cargo-bootimage
   Installed package `bootimage v0.10.3` (executables `bootimage`, `cargo-bootimage`)
```

- さらに `llvm-tools-preview` という rustup コンポーネントも必要なので追加する

```bash
 rustup component add llvm-tools-preview
info: downloading component 'llvm-tools'
info: installing component 'llvm-tools'
 32.0 MiB /  32.0 MiB (100 %)  21.2 MiB/s in  1s ETA:  0s
```

- bootimage を作る

```bash
cargo bootimage
WARNING: `CARGO_MANIFEST_DIR` env variable not set
Building kernel
   Compiling bootloader v0.9.23
   Compiling blog_os v0.1.0 (/Develop/rust/writing-os-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
Building bootloader
    Updating crates.io index
  Downloaded zero v0.1.2
  Downloaded bitflags v1.2.1
  Downloaded toml v0.5.6
  Downloaded rlibc v1.0.0
  Downloaded bit_field v0.10.1
  Downloaded x86_64 v0.14.7
  Downloaded volatile v0.4.4
  Downloaded serde v1.0.116
  Downloaded usize_conversions v0.2.0
  Downloaded fixedvec v0.2.4
  Downloaded xmas-elf v0.6.2
  Downloaded 11 crates (276.7 KB) in 0.85s
  ...(snip)
    Finished release [optimized + debuginfo] target(s) in 8.83s
Created bootimage for `blog_os` at `/Develop/rust/writing-os-rust/target/x86_64-blog_os/debug/bootimage-blog_os.bin`
```

### QEMU 使って起動する

- qemu のインストールから
  - https://www.qemu.org/download/#macos
- 以下のコマンドで起動

```bash
qemu-system-x86_64 -drive format=raw,file=/Develop/rust/writing-os-rust/target/x86_64-blog_os/debug/bootimage-blog_os.bin 
```

![boot helloworld](./image/boot_helloworld.png)

### cargo run で起動する

- いちいちQEMUを起動するのが面倒くさい
- `.cargo/config.toml` に以下を追加

```toml
[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

起動した

```bash
cargo run                                                                                                                                         hiraku.komuro@tcc-pma-000965
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `bootimage runner target/x86_64-blog_os/debug/blog_os`
Building bootloader
    Finished release [optimized + debuginfo] target(s) in 0.04s
Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin`
```

