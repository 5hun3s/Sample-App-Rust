# Rust + React + Tauri Dev Container

Windows本体へRustやNode.jsを入れず、VS CodeのDev Container内で開発するスターターです。Windows版 `.msi` / `-setup.exe` はGitHub Actionsで生成します。

Windows版アプリの起動中は、アクティブなウィンドウのタイトル、実行ファイル名、プロセスID、ウィンドウクラスを5秒ごとに取得します。履歴はメモリ上に直近100件だけ保持され、アプリを終了すると破棄されます。

## 起動コマンド

```
npm run tauri dev
```
