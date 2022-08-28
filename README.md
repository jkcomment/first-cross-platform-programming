# first-cross-platform-programming

## rust-cross-ios
### ビルド関連必要なものをインストール
- Xcodeインストール
- Xcodeビルドツールインストール
```
$ xcode-select --install
```
- cross compileができるようにiOSアーキテクチャ追加
```
$ rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```
- cargo-lipoインストール
```
$ cargo install cargo-lipo
```
### ビルド
- ライブラリファイル生成
```
rust-cross-ios $ cargo lipo --release
```

## rust-cross-web
### ビルドツールインストール
- wasm-packインストール
```
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
- npmインストール
```
$ npm install npm@latest -g
```