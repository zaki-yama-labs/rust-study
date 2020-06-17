第7章 所有権システム
=================

### 7-1 所有権システムの利点

- ガベージコレクタが不要になる
- メモリ安全性がコンパイル時に保証される
- ファイルやロックなどのリソースが使い終わった時点で自動解放できる

7-1-1 ガベージコレクタが不要になる

- ガベージコレクタ:
  - プログラムが動的に確保したメモリ領域の中で不要になった部分を自動的に見つけ出して解放してくれる、メモリ管理のしくみ
  - 実行時の性能に少なからず影響を与える
- Rustは所有権という概念により、コンパイル時の解析で個々のメモリ領域が不要になるタイミングが決定され、それらを解放するコードが自動的に挿入される

7-1-2 メモリ安全性がコンパイル時に保証される

- メモリ安全性とは
  - メモリの2重解放による未定義動作を起こさない
  - 不正なポインタ(ダングリングポインタ)を作らない
- ダングリングポインタ(dangling pointer)
  - 解放済みの領域など無効なメモリを指すポインタ
- 未定義動作
  - Rustの言語仕様で定義されていない振る舞い

### 7-2 所有権システムの概要

所有権システムの役割

1. リソース(メモリ、ファイルディスクリプタなど)の自動解放
  - 解放漏れによるリソースリークの防止
  - 2重解放による未定義動作の防止
2. ダングリングポインタの防止

用語

- 所有権(ownership)
- ムーブセマンティクスとコピーセマンティクス
  - ある変数から別の変数へ値を代入するとき、値の型によってプログラムの意味（セマンティクス）が変わる
  - ムーブセマンティクスは代入によって所有権が移動する。
- 借用(borrow)
  - 参照と借用の実体は同じ
  - 値を指す参照を作ると、所有権の観点からは値を借用していることになる
- ライフタイム(lifetime)
  - 値のライフタイムと参照のライフタイムを区別する必要がある
    - 前者を値のスコープと呼ぶ
  - 値のライフタイム: 値が構築されてから破棄されるまで
  - 参照のライムタイム: 値への参照が使用される期間
- 借用規則
  - コンパイラは以下の規則が守られているか検査することでメモリ安全性を保証する
  1. 参照のライフタイムが値のスコープよりも短い
  2. 値が共有されている間は値の変更を許さない。つまり、ある値Tについて以下のいずれかの状態のみを許す
    - 任意個の不変の参照&Tを持つ
    - ただ1つの可変の借用&mut Tを持つ


7-6 コピーセマンティクス

- 構造体や列挙型にCopyトレイトを実装すると、値がムーブするのではなく、コピーされるようになる
- 構造体や列挙型が以下のすべての条件を満たすときCopyトレイトを実装できる
  1. その型のすべてのフィールドの型がCopyトレイトを実装している
  2. その型自身とすべてのフィールドの型がデストラクタ(Dropトレイト)を実装していない
    - ヒープ領域を使用するデータ型Box<T>, Vec<T>, Stringなどデストラクタを持つ
  3. その型自身がCloneトレイトを実装している