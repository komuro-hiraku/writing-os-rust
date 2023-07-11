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
