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
