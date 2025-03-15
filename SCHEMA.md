# データベース仕様

## users テーブル
- `id`: INTEGER, AUTOINCREMENT
- `name`: TEXT
- `at_id`: TEXT（最低3文字以上, UNIQUE）
- `birthday`: TEXT（`YYYY-MM-DD` 形式で保存）

## tweets テーブル
- `id`: TEXT（UUIDを文字列で保存）
- `created_at`: TEXT（`YYYY-MM-DD HH:MM:SS` 形式）
