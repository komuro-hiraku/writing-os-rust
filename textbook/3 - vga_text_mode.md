# VGA テキストモード

https://os.phil-opp.com/ja/vga-text-mode/

## VGAテキストバッファ

- 25行x80列の2時限配列

|ビット|値|
|:----|:----|
|0-7 | ASCIIコードポイント|
|8-11 | 前景色（フォアグラウンド）|
|12-14 | 背景色（バックグラウンド）|
|15 | 点滅 |

- 最初から 1 byte = 出力文字の ASCIIコード（コードページ437
- 次の 1 byte = 前半4bitで文字色、後半3bitで背景色、最後の1bitで点滅するかどうか
- 色の4bit目は bright bit で明るくするかどうか（文字色
- 点滅は背景色にしかないよ
- Memory Mapped I/O=ハードウェアのテキストバッファを抽象化して、通常のRAM操作との違いを意識しなくておk
    - ただし、RAM操作全部をサポートしているわけではない
- テキストバッファは通常の読み書きをサポートしてるのであんま問題なし

## Rust Module

- 最終的に unsafe な操作をモジュール内に隠蔽して外部からは普通の Rustモジュールとして使うだけでOKにする
- repr is 何
    - https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent


![](./image/vga_textbuffer_sample1.png)

### Volatile

- Bufferに書き込むが読み込めない
    - そのため、コンパイラはVGAバッファメモリにアクセスしてることを知らない
    - したがって文字が出力されるという副作用も知らない
    - 最適化の対象となり省略可能と判断される可能性がある
- `volatile` は最適化の対象から外す命令
    - 専用クレートがある

```toml
[dependencies]
volatile = "0.2.6"
```

- Buffer 定義を修正する

## Global Interface

- static で Writer　を宣言する

```rs
pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe {
        &mut *(0xb8000 as *mut Buffer)
    }
};
```

- compile するとエラー
    - error[E0015]: cannot call non-const fn `ColorCode::new` in statics. note: calls in statics are limited to constant functions, tuple structs and tuple variants
    - error[E0658]: dereferencing raw mutable pointers in statics is unstable. note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
    - コンパイル時に初期化される
    - const evaluator
        - https://rustc-dev-guide.rust-lang.org/const-eval.html
    - そもそも Rust の const evaluator はコンパイル時に生ポインタを参照へ変えることができない
    
### Lazy 静的変数

- `lazy_static` https://docs.rs/lazy_static/1.0.1/lazy_static/
- static の初期化が後回しにされる `static` を定義する `lazy_static!` マクロを提供
    - 最初にアクセスした時に初めて初期化が実行される

```toml
lazy_static = {version = "1.0", features = ["spin_no_std"]}
```

- Immutable な WRITER　なので役に立たない（書き込めないので
- Mutable な static 変数を使うといけるけど、全部が可変になってしまってよろしくない
    - 全部 unsafe になる
- static mut は削除しようという提案がある状態なので良くない

### Spin Lock

- 同期された内部普遍性を得るためにどうするか
    - Mutexを使う（ただし標準ライブラリが使える前提。今回は使えない。標準ライブラリをアンリンクしているので）
- 単純な Mutex＝Spinlock
- `spin` クレートを追加する

### 安全性

- unsafe ブロックは局所的
- それ以外のインタフェースは Rust によってチェックされるので安全安心

## println マクロ

- 標準ライブラリ実装
    - https://doc.rust-lang.org/nightly/std/macro.println.html
```rs
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
```

- `#[macro_export]` クレート全体、外部クレートで使えるようにする属性
- `print!` マクロの呼び出し
    - `format_args!` マクロ呼び出して `fmt::Arguments` 型を作成
    - `$crate::io::_print` に引き渡し
        - いろんな `Stdout` デバイスの実装があるため超複雑
        - VGAバッファに出力したいだけなので、同じ実装だとファットすぎる
- `print`, `println` マクロを修正
- `($($arg:tt)*)` これはマクロ独自の何かか？？

## Panicメッセージ出力

- `println!` マクロができたので Panic時にメッセージを出力できるようになった

## error[E0152] duplicate lang item in crate `core`

- `cargo test` を実行すると上記エラーが出る
    - `panic="abort"` を指定しているのが問題
    - 詳細はこちら https://github.com/rust-lang/cargo/issues/7359
    - 手っ取り早いのは `Cargo.toml`からこの記述を消す
- test 実行しても `_start` が Call されてしまう
    - test_runner が実行されていない
    - 独自テストフレームワークを使っていてこれは `main` を期待するが `#[no_main]` 属性によって main 関数がエントリポイントになっていない
    - 以下で `test_main` （生成されるやつ）をみるように修正

```rs
#![reexport_test_harness_main = "test_main"]
```

- `#[cfg(test)]` で条件付きコンパイル（テストの時だけ

## QEMU を終了

- テストごとにQEMU手動で落とさないとダメ
    - 面倒すぎる
- OSシャットダウンが正攻法
    - ただし APM, ACPIパワーマネジメント標準規格のサポート実装があって複雑
- QEMU には `isa-debug-exit` デバイスがサポートされている
    - `Cargo.toml` に以下を書いて QEMU に引数を渡す
- `iobase=0xf4,iosize=0x04` I/Oポートを指定
    - `0xf4` はx86では通常使われていないポート
    - `iosize=0x0f` は4バイトのポートの大きさを示す

### 終了デバイス

- I/Oポートに値が書き込まれるとそれから終了ステータスを計算して終了する
    - `(value << 1) | 1`
    - 例: value=0 => 1, value=1 => 3
- x86_64 Crate
    - アセンブリ命令を手動で呼び出すのではなく、抽象化された形で実装する

```bash
error: test failed, to rerun pass `--bin blog_os`

Caused by:
  process didn't exit successfully: `bootimage runner /Develop/rust/writing-os-rust/target/x86_64-blog_os/debug/deps/blog_os-7f32dd94eb419a0b` (exit status: 33)
```

- cargo test は exit_code = 0 じゃないとエラーとみなす
- `Cargo.toml` の `package.metadata.bootimage` に以下を追加する

```toml
test-success-exit-code =33
```

- QEMU でテスト実行しテスト終了後に落とすという動作ができた

## コンソール出力

- テスト結果は現状QEMUで表示しているが、テスト終了後に落としてしまってるのでテスト結果は確認できない
- コンソールに出力したい

