第5章 ユーザ定義型
===============

### 5-1 スタック領域とヒープ領域

- スタック領域
  - 関数の引数やローカル変数などが格納されるメモリ領域
  - 複雑なしくみを使わずに管理できるため確保と解放が高速
  - 反面、サイズがあまり大きくない
- ヒープ領域
  - プログラム内で共有されるデータを格納するメモリ領域
  - 管理が複雑、スタックに比べ遅くなる
- Rust は
  - デフォルトで値をスタック老息に置く
  - 例外
    - Boxポインタ (`Box<T>`)
    - `Vec<T>`や`HashMap<K, V>` (要素数が可変のコレクション型)
    - `String`や`OSString` (文字の追加や削除が可能な文字列型)
    - Box以外は実行時に大きさが変更できるデータ構造


### 5-2 標準ライブラリの主な型

5-2-1 Box

- メモリ安全なポインタ
- 特徴
  - 対象のデータをヒープ領域に置く
  - ポインタでありながら、対象のデータを所有する

5-2-2 ベクタ

- プリミティブ型の配列との違い
  - 要素の追加や削除ができる
  - 配列はスタック領域に置かれるが、ベクタはヒープ領域にデータを置く