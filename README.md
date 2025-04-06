# rust-template

## Installation

### 1. 前提環境の構築
以下のコマンドが使用できるようにしておいてください。
- `make`: コマンドの実行用
- `docker-compose`: 環境の構築用

### 2. 開発環境の構築
```sh
# Dockerのビルド
make build

# Dockerの起動
make run

# Dockerの停止
make down

# Docker環境のシェル起動
make shell

# 起動中のDocker環境のシェルへのアクセス
make access
```
