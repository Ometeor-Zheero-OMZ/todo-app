# ビルド用のイメージ
FROM node:lts-buster-slim

# 作業ディレクトリの作成と設定
WORKDIR /usr/src/app

# 依存関係のコピー
COPY package*.json ./

# 依存関係のインストール
RUN npm install

# 残りのアプリケーションファイルをコピー
COPY . .

# 実行するポート番号
EXPOSE 3000

# アプリケーションを実行するコマンド
CMD ["npm", "run", "dev"]