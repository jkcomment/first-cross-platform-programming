# first-cross-platform-programming

## rust-cross-iosビルド
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