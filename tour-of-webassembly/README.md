Tour of WebAssembly
===================

https://tourofrust.com/webassembly/00_ja.html

## メモ

- 10
  - 関数から返すことのできる値は数値のみ (i32/i64, f32/f64)
- 11
  - WASMのプログラムはメモリ領域をバイト列としてエクスポートできる
  - ホスト側(🤔)でそのデータ構造を解釈したり、バイト列を書き込むこともできる
    - プリミティブでない値をやりとりするための手段となる
- 14 テキストのロギング
  - 🤔何もしてないのに `module.instance.exports.memory.buffer` にアクセスできるのはなぜ？
- 15 テキストの読み込み
  - 単純にRustの文法がわからない
    - ptr が返しているものは... `*mut u8` とは...
    - `std::mem::forget` とは

## 疑問

- `--target=wasm32-unknown-unknown` だけではだめで、Cargo.toml に以下を書かないと .wasm が生成されなかった

```toml
[lib]
crate-type = ["cdylib"]
```

- `extern "C"` の `C` ってなんで必要？
