![GitHub all releases](https://img.shields.io/github/downloads/tasuren/projector/total) ![GitHub release (latest by date)](https://img.shields.io/github/v/release/tasuren/projector)

# projector
これは、マインドマップ形式でメモを保存するためのソフトです。  
まだ開発されたばかりで、動作に補償はありません。  
今の所、Windows10とMac(M1)で動作することを確認していて、ダウンロードは[こちら](https://github.com/tasuren/aSynthe/releases)から可能です。  
なお、動作があまり安定していませんが、[ウェブ版](https://projector.tasuren.xyz)もあります。  

## Screenshot
<img width="894" alt="スクリーンショット 2022-09-25 17 55 34" src="https://user-images.githubusercontent.com/45121209/192135786-50aab79c-6500-416c-b21d-0520a0d63a0c.png">

## Builds
ビルドをする前に、`ZenMaruGothic-Regular.ttf`という日本語のフォントのファイルをダウンロードして、`assets`フォルダに入れる必要があります。  
そのフォントのダウンロードは、[こちら](https://fonts.google.com/specimen/Zen+Maru+Gothic?subset=japanese)からできます。
### WebAssembly
1. [trunk](https://trunkrs.dev)をインストールする。
2. `trunk build --release`を実行する。
### Windows
`cargo build --release`を実行する。
### Mac
1. `cargo install cargo-bundle`を実行して、cargo-bundleをインストールする。
2. `cargo bundle --release`を実行する。