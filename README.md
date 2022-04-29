# フランのゲーム

## 創作中のメモ

### ゲーム

プレイヤーの左右移動によって、背景が傾いていいかも。AC4以降のカメラワークみたいな。

## 創作中の愚痴

### プログラミングについて

Git、GitHub覚えましょうねぇ。

エラー文の典型的な書き方みたいなのの文献が簡単に出ても良くない？　
どこもかしこもアルゴリズムばかりだ。貴様もどうせそうなるのだろう？

構造体は引数を連ねる面倒を省くもの、っていうくらいの認識が良い気がする。
~~やっぱプログラミングは関数型だわ。~~

ECSに乗り換えました。関数型パラダイムは理念こそ「正しい」ながら、実行時パフォーマンスを損なう。

### Rustについて

ResultのエラーをStringからユーザー定義に変えようとして、渋々`Box<dyn WError>`を使ったが、`From`トレイトがどうとか言われて面倒になったので、列挙型で定義した。

`*const c_void`へのキャストが`v.as_ptr() as *const _ as *const ::core::ffi::c_void;`なのは見た目がやばい。
`std::mem::transmute`を使うと綺麗になる。

`std::env`とかがどこからでも呼べるの気持ち悪い。

### windows-rsについて

DirectXMathは、ありません！　88888888888クソが！　アフィン変換行列を直打ちする以上の虚無作業はないよ。

COMの作成失敗のError文が「パラメータが間違っています」だけってどうなの？　
このエラー文だけ読んでPCSTRのフィールドに渡すリテラルの末尾はヌル文字でなければならないなんて、誰が分かるの？

`D3D11_RENDER_TARGET_BLEND_DESC.RenderTargetWriteMask`について、C++では`D3D11_COLOR_WRITE_ENABLE`列挙型を使っていたのに、このクレートでは`u8`を指定するようになっている。
構造体自体は定義されているのに、慣例とは違う値を渡さなければならないことがある。
酷いことに、C++版のリファレンスを読んでもｇｇっても`D3D11_COLOR_WRITE_ENABLE_ALL`列挙子の値が書いていない。

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

`CreateFontSetBuilder2`メソッドで`IDWriteFontSetBuilder1`が作られるのは名前の付け方が酷い。

`IDWriteLocalizedStrings::FindLocaleName`のトレイト境界が`IntoParam<'a, PCWSTR>`なんだけど、何がこれを満たしているねん。

`IRestrictedErrorInfo::GetErrorDetails`の引数に`*mut windows::core::bindings::BSTR`が必須で、それが大事なんだが、`bindings`モジュールがprivateという。
どんな開発してんだ。

## ECS

### コンポーネント表

| コンポーネント名 | 概要 |
| ----- | ----- |
| Input | 入力情報を持つ。シングルトン。 |
| FpsMeasure | FPSを測定する。シングルトン。 |
| PlayerAnimation | 速度に対して画像を変える。 |
| PlayerInput | 入力に対して自機を動かす。マーカー。 |
| Position | 物体の位置。 |
| RestrictRect | 物体の位置を制限する。 |
| SamePosition | 物体の位置を特定の物体と同じにする。 |
| Sprite | スプライト。 |
| Text | 文章。現状は最前面に描画される。 |
| Velocity | 物体の速度。 |

### 各コンポーネントの影響関係

現状はコンテナをHashMapで管理しているため、エンティティはあるコンポーネントを高々一つしか持てない。
そのため、影響元と先とで、列挙する方を調整すると、計算負荷を抑えられるだろう。
列挙する方を左に書き、影響の方向を矢印で表した。

* (PlayerInput, Input) -> Velocity
* FpsMesure -> Text
* Velocity -> Position
* RestrictRect -> Position
* SamePosition <- (Position, *Entities*)
* SamePosition -> Position
* (PlayerAnimation, Velocity) -> Sprite
* Position -> Sprite

### 各エンティティの実装

Player

* PlayerInput
* Velocity
* Position
* Sprite
