 

 <img src="horizontallockup_primary_JWbDwnP.png" alt="logo" width="120">

# UsefulDevice

マルチプラットフォーム対応のデバイス制御アプリケーション

## 注意事項

⚠️ **重要**: CC notfound ビルドコマンドにapkを入れ忘れるな

## セットアップ

### Rustツールチェインの準備

対象プラットフォーム用のRustターゲットを追加します：

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

### iOS・macOS向け

```bash
# iOS
rustup target add aarch64-apple-ios

# macOS
rustup target add aarch64-apple-darwin
```

## ビルド・実行方法

### Android向けビルド

```bash
# リリースビルド
cargo apk build --release --target aarch64-linux-android

# エミュレータでの実行
adb devices
cargo apk run --target aarch64-linux-android --lib
```

## Windows開発環境のセットアップ

### 1. LLVM のインストール

```bash
winget install --scope machine LLVM.LLVM
```

インストール後、システムのPATH環境変数にLLVMが追加されていることを確認してください。

### 2. Bonjour のインストール

ネットワーク機能に必要な場合があります。Apple公式サイトまたはiTunesと一緒にインストールできます。

### 3. Android Studio のセットアップ

1. [Android Studio](https://developer.android.com/studio?hl=ja) をダウンロード・インストール

2. SDK Manager で以下のコンポーネントをインストール：

   **SDK Platforms:**
   - Android SDK Platform 30 (API Level 30)
   - Sources for Android 30

   **SDK Tools:**
   - Android SDK Build-Tools 36
   - NDK (Side by side) - 最新バージョン
   - Android SDK Command-line Tools (最新版)
   - Android Emulator
   - Android SDK Platform-Tools

3. 環境変数の設定

   以下の環境変数を設定してください：
   ```
   ANDROID_HOME=C:\Users\[ユーザー名]\AppData\Local\Android\Sdk
   ANDROID_NDK_ROOT=%ANDROID_HOME%\ndk\[バージョン]
   ```

   PATH に以下を追加：
   ```
   %ANDROID_HOME%\platform-tools
   %ANDROID_HOME%\tools
   %ANDROID_HOME%\tools\bin
   ```

## プロジェクト構成

- `android_lib/` - Androidの機能をRustで呼び出せるようにする開発フォルダ
- `src/` - メインのRustソースコード
- `src/client/` - クライアント機能
- `src/server/` - サーバー機能
- `ui/` - Slint UIファイル


