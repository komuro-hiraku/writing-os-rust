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
   Compiling blog_os v0.1.0 (/Users/hiraku.komuro/Develop/rust/writing-os-rust)
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

