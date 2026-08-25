# PORTING

このリポジトリは現時点でプラットフォーム非依存のRustライブラリ
クレート(`crates/kagaku-core`)のみで構成されており、OS依存のコードは
含まれていない。`cargo build --workspace --release`が通る環境であれば
Windows/Linux/macOSいずれでも動作する。

外部クレート依存は`thiserror`のみ(標準的なcrates.io依存、pathベースの
sibling repo依存は無し)。他リポジトリ(`open-cg-cad`等)からの利用時も
特別な配置ルールは不要。
