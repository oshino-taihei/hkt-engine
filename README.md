# hkt-engine

## ローカル開発環境

### 前提

- 開発はVisual Studio Code(以下、VSCode)で行うため、VSCodeがインストール済みであること
- Dockerがインストール済みであること
- VSCodeのRemote Developmentプラグインがインストール済みであること

### 構築手順

  1. 本プロジェクトをローカルにgit clone
  2. VSCodeで本プロジェクトフォルダを開く
  3. 右下に[Folder contains a dev container configuration file...]ポップアップが表示された場合「Reopen in Container」ボタンをクリックする。
  表示されなかった場合はステータスバー左下の[><]メニューをクリックし、[Remote-Containers: Reopen in Container]を選択する
  4. DevContainerが起動し、Remote-Containerに接続されることを確認

### 動作確認

テスト実行
```bash
cargo test
```
以下のように<code>test result: ok</code>と表示されればOK。
```
test result: ok. <n> passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
