![GitHub release (latest by date)](https://img.shields.io/github/v/release/tasuren/projector)

# projector
これは、マインドマップ形式でメモを保存するためのウェブツールです。  
まだ開発されたばかりで、動作に補償はありません。  
ウェブツール本体と詳細は[ここ](https://projector.tasuren.xyz/information.html)から開けます。

## Screenshot
<img width="894" alt="スクリーンショット 2022-09-25 17 55 34" src="https://user-images.githubusercontent.com/45121209/192135786-50aab79c-6500-416c-b21d-0520a0d63a0c.png">

## Builds
### WebAssembly
1. [trunk](https://trunkrs.dev)をインストールする。
2. `trunk build --release`を実行する。
### Windows
`cargo build --release`を実行する。
### Mac
1. `cargo install cargo-bundle`を実行して、cargo-bundleをインストールする。
2. `cargo bundle --release`を実行する。