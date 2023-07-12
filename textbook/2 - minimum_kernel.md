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