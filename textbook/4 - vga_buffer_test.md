# Test

- 普通に実行すると死ぬ
```bash
error[E0463]: can't find crate for `test`
```
- 標準ライブラリに依存しているため。 `no_std` な我らに実行する術がない
- test クレートを持ってくることもできるけど、煩雑なハックが必要

## custom_test_frameworks

- 外部ライブラリに依存しない
- ただし機能は薄いので欲しければ自分で作る必要がある
- `#[should_panic]` 属性はパニックを検知するためにスタックアンドワインドを使う
    - これは無効化しているのでそもそも動かない
- 自分で test_runner を定義してあげる

```rs
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
```

- `Running XXX tests` というデバッグメッセージを表示して与えられた関数を呼び出すだけ
    - `#[cfg(test)]` でテストの時だけ使うように宣言
- custom_test_framework は自動的に main 関数を生成するが `#[no main]` によって無効化されている
- `_start` を呼んでしまう

```rs
#![reexport_test_harness_main = "test_main"]
```

- test 時の main 関数の名前を明示的に指定

```rs
#[cfg(test)]
test_main();
```

- テスト時だけ呼び出すように
- テスト実行時の挙動
    - `_start` 実行
    - `test_main` 実行
    - `test_runner` 関数を実行
    - `test_runnner` から　`test_main` へ Return
    - `test_main` から `_start` へ Return
    - `_start` へ制御が戻る（この場合 `loop()` により無限にぐるぐる回る）
- テスト終了時に `loop()` で終了しないのは困る
- テスト終了後に QEMU を閉じたい
- そこで Serial I/O を利用したメッセージ送信による QEMU の終了

## Serial Port

- `x86_64` クレートを使えばOK
    - `isa-debug-exit`
    - 書き込む先のポート情報（ `iobase` 
        - ここでは `0xf4` = x86 において通常使われないポート
    - 書き込み先のポートの大きさ（ `iosize`
        - ここでは `0x04` = 4 bytes
- 終了ステータス
    - `(value << 1) | 1`
        - 例: value = 0 の場合 (0 << 1) | 1 で 1
        - 例: value = 1 の場合 (1 << 1) | 1 で 3 
- 適当にやると cargo test で失敗する
    - cargo test が期待する終了コードは 0 が成功でそれ以外は失敗であるため
    - `bootimage` のメタデータ設定に `test-success-exit-code` を追加する
        - (0x10 << 1) | 1 = 33
        - bootimage が 33 を 0 にマッピングする