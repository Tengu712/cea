# フランのゲーム

## 創作中のメモ

### ゲーム

プレイヤーの左右移動によって、背景が傾いていいかも。AC4以降のカメラワークみたいな。

### ECS

* エンティティ：コンポーネント間を紐づけるための識別番号。
* コンポーネント：エンティティのデータ。システムを適応するためのマーカー。
* システム：ロジック。状態をこねこねする。

CContainerの改善案。

* Componentを列挙したVec
* EntityIDとインデックスを結びつけるHashMap
* 空きスロットを記憶するDeque

マーカーコンポーネントを定義する、そのコンポーネントのコンテナを作る、システムを定義する、コンポーネントとシステムを追加する、という作業工程が面倒。
とにかく、内側の要素が外側の要素にアクセスできない。システムにスクリプトをぶち込んで、どうにかマーカーを抽象化したい。マーカーは文字列でいい？

コンポーネントがある限りシステムは作動する。
エンティティにはActive、Disactive、Removedの状態がある。


## 創作中の愚痴

### プログラミングについて

Git、GitHub覚えましょうねぇ。

エラー文の典型的な書き方みたいなのの文献が簡単に出ても良くない？　
どこもかしこもアルゴリズムばかりだ。貴様もどうせそうなるのだろう？

構造体は引数を連ねる面倒を省くもの、っていうくらいの認識が良い気がする。
~~やっぱプログラミングは関数型だわ。~~

ECSに乗り換えました。関数型パラダイムは理念こそ「正しい」ながら、実行時パフォーマンスを損なう。

おいDirect3D11。Zバッファとアルファブレンドが共存できないなんて聞いてないぞ。おかげでZソートすることになったじゃねえか。計算負荷。

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

なんでもかんでも参照可能・変更可能というやばい実装をしている。突貫工事だから仕方ない。

### Components

| Genre | Components Name | Outline |
| ----- | ----- | ----- |
| counter | Counter | カウンター。毎フレームインクリメントされる。 |
| fpsmeasure | FpsMeasure | FPS、それを算出するためのデータ。 |
| movement| Position | 三次元座標。 |
| | RemoveRect | 存在可能な三次元範囲。 |
| | RestrictRect | 三次元範囲制限。 |
| | SamePosition2D | 特定の物体の座標と同じ座標にする。 |
| | Velocity | 速度。 |
| graphic | Sprite | スプライト。描画用データ。 |
| | Text | テキスト。描画用データ。 |
| | ValueText | カウンターの値を特定の書式でテキストに反映する。 |

### System

詳しくはRustDocを見てほしい。

* counter
  * () -> Counter
  * ValueText, Counter -> Text
  * () -> FpsMeasure, Text
* movement
  * Velocity -> Position
  * RemoveRect, Position -> *remove entity*
  * RestrictRect -> Position
  * SamePosition2D, Position -> Position
  * Position -> Sprite

### Entity Implementation

Fps

* FpsMeasure
* Text

Title text

* Counter
* Text

Player

* Position
* RestrictRect
* Sprite
* Velocity
