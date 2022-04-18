# 秘封俱楽部のノベルゲー

## 創作中の愚痴

### プログラミングについて

エラー文の典型的な書き方みたいなのの文献が簡単に出ても良くない？　
どこもかしこもアルゴリズムばかりだ。貴様もどうせそうなるのだろう？

### Rustについて

ResultのエラーをStringからユーザー定義に変えようとして、渋々`Box<dyn MyError>`を使ったが、`From`トレイトがどうとか言われて面倒になったので、列挙型で定義した。

`*const c_void`へのキャストが`v.as_ptr() as *const _ as *const ::core::ffi::c_void;`なのは見た目がやばい。

### windows-rsについて

COMの作成失敗のError文が「パラメータが間違っています」だけってどうなの？　
このエラー文だけ読んでPCSTRのフィールドに渡すリテラルの末尾はヌル文字でなければならないなんて、誰が分かるの？

開発途上だからリファレンス通りの実装じゃなかったり文献が少なかったりは許すからよ、サンプルプログラムのフォーマットぐらいしてくれ。
クソ雑魚の俺がContributorになってやろうか？

SwapchainからBackbufferを取得しようとしたら、「the trait `windows::core::Interface` is not implemented for `()`」って言われた。
正解は下のコードだが、このエラー文からじゃ型推論エラーとは分からんやろ。

```rust
// 誤
swapchain.GetBuffer(0)
// 正
swapchain.GetBuffer::<ID3D11Texture2D>(0)
```
