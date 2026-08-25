# PORTING

このリポジトリはRustワークスペースで構成されており、OS依存のコードは
含まれていない。`cargo build --workspace --release`が通る環境であれば
Windows/Linux/macOSいずれでも動作する。

## クレート構成

- `crates/kagaku-core`: プラットフォーム非依存の純粋ロジック
  (`molecular_formula`・`otc_ingredient`・`kampo_formula`・
  `cold_medicine_formulation`・`vitamin_supplement`の各モジュール)。
  外部クレート依存は`thiserror`のみ(標準的なcrates.io依存)。
- `server`: `RPoem`ベースのHTTPサーバー。`kagaku-core`の全機能をJSON
  APIとして公開し、日英併記のWeb UI(`server/src/index.html`)を配信する。
  既定待受先は`127.0.0.1:4702`(環境変数`OPEN_KAGAKU_SERVER_BIND`で変更可)。

いずれもpathベースのsibling repo依存は無く、他リポジトリ(`open-cg-cad`等)
からの利用時も特別な配置ルールは不要。

## デプロイ(参考: easy-web.tokyo実績)

`https://easy-web.tokyo/open-kagaku/`へ以下の構成で実デプロイ済み:

- VPS上へ`git clone`(sibling path依存が無いため、open-cg-cadと異なり
  追加のclone/symlinkは不要。`RPoem`は既存の共有クローンを使用)。
- systemdサービス`installer/open-kagaku.service`をポート8106で新設
  (`OPEN_KAGAKU_SERVER_BIND=127.0.0.1:8106`)、`systemctl enable --now`
  で自動起動を有効化。
- `open-web-server`側の`domains.toml`に`path_prefix = "/open-kagaku"`
  (`strip_prefix = true`)を追加してリバースプロキシ。

他環境へポーティングする場合も、このサーバー/ポート/`domains.toml`の
組み合わせパターンをそのまま踏襲できる。
