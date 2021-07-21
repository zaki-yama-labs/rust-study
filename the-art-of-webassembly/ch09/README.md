# Chapter 9: Optimizaing Performance

<!-- TOC -->

- [Using a Profiler](#using-a-profiler)
  - [Chrome Profiler](#chrome-profiler)

<!-- /TOC -->

この Chapter で学ぶこと

- プロファイラツールの使い方
- WebAssembly と似たような JS とのパフォーマンス比較
- WebAssembly のパフォーマンスを向上させる戦略
  - 関数のインライン化、乗算・除算をビットシフトに置き換え、Dead Code Elimination (DCE)
- `console.log`と`Date.now`を使ったアプリケーションパフォーマンスの測定
- V8 の中間表現(Intermediate Representation: IR)バイトコード

## Using a Profiler

- 異なるタイプの最適化の間でトレードオフを行う必要があることも多い
  - たとえば、ユーザーがアプリケーションを可能な限り早く使い始められるよう TTI を向上させるか、ピーク時のパフォーマンスを優先するか
- DevTools の Performance タブの概要

### Chrome Profiler

- JavaScript Heap Memory
  - JS では GC があるのでメモリを気にする必要がないと考える開発者もいるが、残念ながらそうではない
    - GC されるより速くオブジェクトを作成している可能性もあるし、必要以上にオブジェクトへの参照を持っており、JS からは削除していいのかわからない状態になってる可能性もある
  - ヒープメモリについては、2.2MB ぐらいまで増加した後 GC が起き、1.2MB ぐらいまで減少していた
- Following the Memory Allocation
  - ヒープの増加が一貫していることから、毎フレームレンダーごとにメモリ割り当てが行われている可能性が高いと推測される
  - WebAssembly の関数呼び出しより `ctx.putImageData` の方がヒープ増加に寄与していた
    - これはビルトイン関数なので最適化のしようがない
    - メモリアロケーションが問題なのであれば別の手段を考える
- Frames
  - 18fps ぐらい（自分の環境だと 30fps ぐらい)
- Bottom-Up
  - Bottom-Up タブはこういったデータが見られる
    - アプリ内で呼ばれた関数
    - それらの実行時間の total time
    - Self Time (関数からさらに呼ばれた関数の実行時間を除外した、その関数自身の実行時間)
  - 🤔 "It’s a bit disappointing that Chrome doesn’t indicate which function it calls inside the WebAssembly module" と書いてるけど、自分の環境だと `wasm-function[4]` みたいな表示だった & Sources タブに飛んでだいたいどの関数かあたりつけることはできた