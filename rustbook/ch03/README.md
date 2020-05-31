第3章 クイックツアー
=================

- バイトニックソート
- ほとんどのソートアルゴリズムは、計算量が「要素数」と「要素の並び方」の両方に依存する
  - 例：クイックソートの平均計算量は `O(n logn)` だが、最悪で `O(n^2)`
- バイトニックソートの特徴
  - 計算量が要素数だけで決まり、常に `O(n logn)`
  - 容易に並列化できる。その場合 `O(logn)`
  - 欠点：要素数が2のべき乗でないとソートできない

- 3-4
  - ジェネリクス
  - 列挙型
  - `Result<T, E>`
- 3-5
  - 構造体
  - 関連関数 (別の言語でいうクラスメソッド、らしい)
  - クロージャ

### 3-4

- `rustc --explain [エラーコード]` とすると、該当エラーのドキュメントが開く
- 列挙型 (enum)
  - 値をバリアントと呼ぶ
- 3-4-7 エラーを返す
  - 組み込みの `Result<T, E>` 型は `Ok(成功時の値)` と `Err(失敗時の値)` の2つのバリアントを持つ


### 3-5 第3段階：クロージャでソート順をカスタマイズ

```rust
|a, b| a.age.cmp(&b.age)

// は以下と同等

fn comparator(a: &&Student, b: &&Student) -> std::cmp::Ordering {
  a.age.cmp(&b.age)
}
```

`std::cmp::Ordering` は `Ordering::Less, Ordering::Equal, Ordering::Greater` 

`then_with()` ?

3-5-8 イテレータとコレクタ

```rust
rng.sample_iter(&Standard).take(n).collect()
```

- `take(n)` は先頭のn個の要素を返すイテレータを生成する
- `collect()` はコレクタ。イテレータから値を取り出し、ベクタやハッシュマップにする

### 3-6 並列処理

- 3-6-4 SyncトレイトとSendトレイト
  - `Sync` トレイトを実装している場合、この型の値は共有された参照を通じて複数のスレッドから並列に使われたとしても、必ずメモリ安全であることを意味する
  - `Send` トレイトを実装している場合、この型の値はスレッド間で安全に受け渡しできる


### 3-7 ベンチマークプログラム

- 🤔 `expect()`
  - `let bits = u32::from_str(&n).expect("error parsing argument");`
