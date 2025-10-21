# Shrimp API

ランダムなエビの情報を返すRust製のAPIサーバーです。

## 機能

- 30種類のエビのデータベース
- ランダムなエビ情報を取得するRESTful API
- JSON形式でのレスポンス

## エビのデータ構造

各エビには以下の情報が含まれています：

- **id**: エビのID
- **name**: 英語名
- **japanese_name**: 日本語名
- **species**: 学名
- **size_cm**: サイズ（センチメートル）
- **habitat**: 生息地
- **color**: 色
- **diet**: 食性
- **lifespan_years**: 寿命（年）
- **fun_fact**: 豆知識

## 技術スタック

- **Rust** - プログラミング言語
- **Axum** - 非同期Webフレームワーク
- **Tokio** - 非同期ランタイム
- **Serde** - JSON シリアライゼーション
- **Rand** - ランダム生成

## セットアップ

### 必要要件

- Rust 1.70以上
- Cargo

### インストール

```bash
# リポジトリをクローン
git clone <repository-url>
cd claude-code-web-test

# 依存関係をインストールしてビルド
cargo build --release
```

## 使用方法

### サーバーの起動

```bash
cargo run
```

サーバーは `http://0.0.0.0:3000` で起動します。

### APIエンドポイント

#### ヘルスチェック

```bash
GET /
```

**レスポンス:**
```
Shrimp API is running!
```

#### ランダムなエビを取得

```bash
GET /api/shrimp/random
```

**レスポンス例:**
```json
{
  "id": 2,
  "name": "Mantis Shrimp",
  "japanese_name": "シャコ",
  "species": "Odontodactylus scyllarus",
  "size_cm": 18.0,
  "habitat": "Coral reefs in Indo-Pacific",
  "color": "Rainbow (green, blue, red, orange)",
  "diet": "Mollusks, crabs, and fish",
  "lifespan_years": 20.0,
  "fun_fact": "Has the fastest punch in the animal kingdom at 50 mph!"
}
```

### curlコマンドの例

```bash
# ヘルスチェック
curl http://localhost:3000/

# ランダムなエビを取得
curl http://localhost:3000/api/shrimp/random
```

## 含まれるエビの種類

1. チェリーシュリンプ (Cherry Shrimp)
2. シャコ (Mantis Shrimp)
3. ヤマトヌマエビ (Amano Shrimp)
4. ゴーストシュリンプ (Ghost Shrimp)
5. クリスタルレッドシュリンプ (Crystal Red Shrimp)
6. テッポウエビ (Pistol Shrimp)
7. ブルーベルベットシュリンプ (Blue Velvet Shrimp)
8. クルマエビ (Kuruma Prawn)
9. バンブルビーシュリンプ (Bumblebee Shrimp)
10. ボタンエビ (Spot Prawn)
11. ウシエビ (Tiger Shrimp)
12. バナメイエビ (Whiteleg Shrimp)
13. オレンジサクラシュリンプ (Orange Sakura Shrimp)
14. イエローシュリンプ (Yellow Shrimp)
15. グリーンジェイドシュリンプ (Green Jade Shrimp)
16. ブラックローズシュリンプ (Black Rose Shrimp)
17. スノーボールシュリンプ (Snowball Shrimp)
18. ブルーボルトシュリンプ (Blue Bolt Shrimp)
19. タンジェリンタイガーシュリンプ (Tangerine Tiger Shrimp)
20. キャメルシュリンプ (Camel Shrimp)
21. ハーレクインシュリンプ (Harlequin Shrimp)
22. ペパーミントシュリンプ (Peppermint Shrimp)
23. バンブーシュリンプ (Bamboo Shrimp)
24. ヴァンパイアシュリンプ (Vampire Shrimp)
25. スカンククリーナーシュリンプ (Skunk Cleaner Shrimp)
26. ファイアシュリンプ (Fire Shrimp)
27. グラスシュリンプ (Glass Shrimp)
28. レッドリリシュリンプ (Red Rili Shrimp)
29. カーボンリリシュリンプ (Carbon Rili Shrimp)
30. ブルードリームシュリンプ (Blue Dream Shrimp)

## 開発

### テスト

```bash
cargo test
```

### リリースビルド

```bash
cargo build --release
```

実行ファイルは `target/release/shrimp-api` に生成されます。

## ライセンス

MIT
