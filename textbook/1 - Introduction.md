# Introduction

[フリースタンディングな Rust バイナリ](https://os.phil-opp.com/ja/freestanding-rust-binary/) を読みながら写経

## 標準ライブラリを無効化

- そもそも `println!()` すら使えなくするという発想をしたことがなかった
- 標準ライブラリを無効にする

```rust
#![no_std]
```

- これだけで `println!()` すらコンパイルに通らなくなる。手足を自らしばっていっているようだ

## Panic Handler

```bash
$ cargo build
error: `#[panic_handler]` function required, but not found
error: language item required, but not found: `eh_personality`
```

- Default では `panic_handler` はデフォルトで定義されている
- 標準ライブラリを潰してるので自分で定義しないといけない
- `PanicInfo` パラメータはパニックした際の詳細情報を含む
    - https://doc.rust-lang.org/nightly/core/panic/struct.PanicInfo.html
- `!` Never型
    - https://doc.rust-lang.org/nightly/std/primitive.never.html

## `eh_personality` Language Item

- Language Item = コンパイラによって内部的に必要とされる特別な関数や型
- `#[lang="copy"]` これは Copy トレイトはどの型がコピーせまんティクスを持っているかをコンパイラに伝える Language Item の例
- `eh_personality` はスタックアンワインドを　実装するための関数を定義する
- スタックアンワインドってなんだ？
    - https://www.bogotobogo.com/cplusplus/stackunwinding.php
    - https://learn.microsoft.com/ja-jp/cpp/cpp/exceptions-and-stack-unwinding-in-cpp?view=msvc-170
    - > catch ステートメントに到達すると、throw ステートメントおよび catch ステートメントの間のスコープ内のすべての自動変数は、スタック アンワインドと呼ばれるプロセスで破棄されます。
    - なるほど〜？
    - Rust の文脈だと Panic したときにスタックにある変数のデストラクタを自動的に実行する
- スタックアンワインドは結構複雑な上、OS特有のライブラリを使うので1からKernelを書く場合にはちょっと困る
    - Linuxだと https://www.nongnu.org/libunwind/
    - Windowsだと https://learn.microsoft.com/en-us/windows/win32/debug/structured-exception-handling

## Unwind を無効化する

- `Cargo.toml` に以下を追加
- Panic 時に何もせずにAbortする

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

## error: requires `start` lang_item

- `start` language item が存在しない
    - Rust Runtimeのエントリポイント
    - https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73
    - C Runtime がここの Runst Runtimeエントリポイントを呼び出すというお決まり
- 通常 `main` 関数実行前に Runtime の初期化等が実行される。Stackの作成とか諸々
- `crt0` = C runtime zero
- この辺のRuntimeエントリポイントが実行されて初めて `main` が呼び出される
- `start` language item は実装できない。なぜならば `crt0` を必要とするため
- `crt0` エントリポイントを直接上書きする
- `#![no_main]` を追加して `main` を消す

```rust
#![no_std]  // 標準ライブラリを使わない
#![no_main] // 標準エントリポイントを使わない
```

### `#[no_mangle]`

- 名前修飾を無効化
- 通常Rustコンパイラは関数名をそのまま使ってシンボル生成せずに、ユニークな値を関数につける（Human readableではない
-Linker に関数の名前を伝える必要があるので、Human Readableで明示的にわかりやすい関数名にしてもらわないと困る
- そのため修飾を無効
- `_start` はシステムRuntimeの事実上のデフォルトエントリポイント名

## リンカエラー

- cc で以下のエラーが発生する

```bash
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" […]
  = note: ld: entry point (_main) undefined. for architecture x86_64
          clang: error: linker command failed with exit code 1 […]
```

- `main` が見つからないエラー（消してるしそれはそう
- `_start` を明示的に指定する必要があるが、 macOS の場合すべての関数に prefix `_` がつくので `__start` となるらしい（へえ
- `-e` はリンカ引数

```bash
cargo rustc -- -C link-args="-e __start -static --nostartfiles"
```

- `cargo rustc`: rustc への引数を渡してコンパイルできるらしい
- `-C link-args`: スペース区切りリストを引数として取るオプション
- `-static`: macOS は静的バイナリのリンクをサポートしてない。これを無視
- `-nostartfiles`: macOSはデフォルトで `crt0` にリンクしようとするためエラーとなる。Cの起動ルーチンをリンクしないようにするため指定する
