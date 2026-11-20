# 設計思想＆開発方針＆開発環境ルール(open-kagaku)

作業ドライブは`F:\runo`。この節は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を
正本とし、各プロジェクトへコピーして同期する既存の運用ルール継承方針に
準じる(実機検証必須・正直な開示徹底等の全リポジトリ共通ルールは、
詳細をここに複製せず`open-raid-z/CLAUDE.md`を参照すること)。
GitHubリポジトリ: [aon-co-jp/open-kagaku](https://github.com/aon-co-jp/open-kagaku)。

**開発開始日: 2026-08-25**。`open-cg-cad`のプラグイン(姉妹プロジェクト)
としてユーザーから提案された(「open-cg-cadのプラグインとしてopen-kagaku
というリポジトリを開発して、薬品名を書いたら…化学式を表示する機能と
猛毒から毒性を取り除く機能などを搭載して…風邪薬シミュレーター…」)。

## ⚠️ 最重要制約(必ず全ドキュメント・コード・UIに一貫して明記すること)

**本リポジトリは医療機器・医薬品ソフトウェアではなく、教育・研究・
設計検討用の参考ツールに過ぎない。実際の服薬判断・処方・製剤設計には
薬剤師・医師等の有資格者によるレビューが必須であり、本ツールの出力を
根拠に服薬量を決定してはならない。**

## HANDOFF

- **2026-08-25(続き6) easy-web.tokyoへ実際にデプロイして公開(ユーザー
  指示②)**: `/root/easy-web.tokyo/open-kagaku`へ`git clone`。
  依存はcrates.ioの`thiserror`のみでsibling path依存が無いため、
  open-cg-cadと違いVPS側の追加clone/symlinkは不要だった(`RPoem`は
  既存の共有クローンをそのまま使用)。ポート`8106`を割り当て
  (`installer/open-kagaku.service`新設、`OPEN_KAGAKU_SERVER_BIND=
  127.0.0.1:8106`)、`systemctl enable --now`で起動、`/healthz`を
  VPS内部から確認。`domains.toml`へ`path_prefix = "/open-kagaku"`
  (`strip_prefix = true`)を追加(`/v1`の追加専有は不要、相対パス
  設計のおかげ)。`systemctl restart open-web-server`で反映後、実際に
  外部から`https://easy-web.tokyo/open-kagaku/`(200、タイトル確認)・
  `POST https://easy-web.tokyo/open-kagaku/v1/molecular-formula`
  (200、aspirinの分子式C9H8O4を正しく返す)を確認した——本物の
  インターネット経由での公開を実証。`open-kagaku.service`は`enabled`
  (再起動後も自動起動)。
  - 次にすべきこと: open-cg-cad優先順位④(WebGL/WebGPU等)は
    `open-cg-cad/CLAUDE.md`参照。open-kagaku自体の拡充は次回課題。

- **2026-08-25(続き5) open-cg-cadとの相互リンクを実装(ユーザー指示
  「①open-cg-cadからopen-kagakuへの導線設置」)**: `server/src/
  index.html`に、open-cg-cad側が読む`open-cg-cad.openKagakuBase`
  localStorageキーへ自分自身のURLを書き込む処理と、open-cg-cad側が
  書き込む`open-kagaku.openCgCadBase`キーを読んでリンク先を上書きする
  処理を追加(open-english⇄open-cg-cadの既存パターンと同型)。
  open-cg-cad側にも同時に対応するリンクを追加した(詳細は
  `open-cg-cad/CLAUDE.md`参照)。実際に両サーバーを起動し、`curl`で
  両ページに互いへのリンク要素が実在することを確認した。
  - 次にすべきこと: easy-web.tokyoへの実デプロイ(ユーザー指示②)。

- **2026-08-25(続き4) 入力欄の区切り文字を柔軟化(ユーザー指示「カンマ
  区切りもピリオドや、,区切りも、半角も全角スペース区切りも半角
  ハイフンも全角ハイフン区切りも、半角アンダーラインも全角アンダー
  ライン区切りも対応して」への対応)**: `server/src/index.html`に
  `splitList`(半角/全角カンマ・句読点・ピリオド・中点・セミコロン・
  半角/全角ハイフン・半角/全角アンダーライン・半角/全角スペース・改行を
  区切り文字として許容)を新設し、漢方処方名・化学成分名の入力欄で使用。
  **正直な開示・設計上の注意点**: 適応分類欄(`recommend-categories`)と
  ビタミンのmg入力欄には、`cold_flu_early_stage`のようにアンダーライン
  自体を値の一部として使う識別子が入るため、`splitList`をそのまま
  適用するとハイフン・アンダーラインで誤分割されてしまう実バグに
  なることに気づき、ハイフン・アンダーラインを含まない別関数
  `splitCategoryList`を用意して使い分けた(全欄で同じ関数を使う安易な
  実装を避けた)。実際にブラウザ上で`splitList`/`splitCategoryList`を
  直接呼び出し、カンマ・全角カンマ・ピリオド・半角/全角ハイフン・
  半角/全角アンダーライン・半角/全角スペースの全パターンで正しく
  分割されること、および`cold_flu_early_stage`が誤分割されずに保持
  されることを確認した。

- **2026-08-25(続き3) HTTPサーバー+UI新設、実在するハイブリッド処方
  (柴朴湯・清肺湯)をDBへ追加**:
  1. **`server`クレート新設**: `open-cg-cad/server`と同じ`RPoem`ベースの
     最小構成パターンを踏襲(既定`127.0.0.1:4702`)。`kagaku-core`の
     全機能(化学式表示・OTC薬DB一覧・漢方処方DB一覧・風邪薬配合
     チェック・甘草重複チェック・単一処方推奨・ビタミン過剰摂取
     チェック)をJSON API化し、`index.html`から呼び出す簡易UI(日英
     併記)を実装。open-cg-cadへの戻りリンクも設置(相互連携の一環)。
  2. **実在するハイブリッド漢方処方2件をDBへ追加(ユーザーの質問への
     回答をGoogle検索で確認・記録)**: ユーザーから「喉と気管支にも
     効く良い所どりなハイブリッド漢方薬はあるか」「喉と肺の両方に良い
     漢方薬」と質問があり、検索の結果、**柴朴湯**(小柴胡湯+半夏厚朴湯の
     合方、構成生薬10種を確認済み)と**清肺湯**(16種類の生薬から成る
     処方、ツムラの公開処方情報で構成生薬を確認済み)がそれぞれ実在
     することを確認し、正確な構成生薬とともにDBへ追加した(既存の
     麦門冬湯と合わせ、喉+気管支の両方をカバーする処方が計3種に)。
  3. **検証結果**: `cargo test --workspace --release`**34件全green**
     (既存28件+新規6件、regression無し)、`cargo clippy --workspace
     --all-targets --release -- -D warnings`警告0件。実際にサーバーを
     起動し、`curl`で全エンドポイント(healthz・分子式・漢方重複
     チェック・単一処方推奨・ビタミンチェック)を実際にHTTP経由で確認、
     実ブラウザ(Claude Browserツール)でUIの化学式検索・単一処方推奨
     ボタンが正しく動作することも確認した。
  - 次にすべきこと: open-english/open-cg-cadとの相互リンク(現状は
    open-cg-cadへの一方向リンクのみ設置、open-cg-cad側からの導線は
    未設置)、easy-web.tokyoへのデプロイ、漢方処方DBのさらなる拡充。

- **2026-08-25(続き2) 単一処方への統合推奨機能を追加(ユーザー指示
  「喉や声に良い漢方薬と肺や気管支に良い漢方薬を別々に飲むと甘草が
  重なって良くないので、その両方の漢方成分の入った二つが一つになった
  漢方薬の方をオススメする機能」への対応)**:
  1. **事実確認**(Google検索): 麦門冬湯が実際に「喉の乾燥」と「乾いた
     咳(下気道症状)」の両方に伝統的に用いられる処方であること、
     麻杏甘石湯が気管支炎・喘息等の気管支症状に用いられる処方(構成生薬
     麻黄・杏仁・甘草・石膏)であることを確認した。**「心臓に良い漢方薬」
     という分類は、本データベースのどの処方についても文献的裏付けを
     確認できなかったため意図的に含めていない**(根拠の無い効能を
     主張しないため、正直な開示)。
  2. **実装内容**: `KampoFormula`に`indication_categories`
     (`IndicationCategory::{Throat, Respiratory, Nasal,
     ColdFluEarlyStage}`)を追加、既存4処方に分類を付与しつつ新規に
     麻杏甘石湯(気管支)を追加(計5処方)。`recommend_combined_formula`
     が、要求された適応分類の組み合わせ(例: 喉+気管支)を単独でカバー
     する既存処方(この場合は麦門冬湯)を検索して推奨する——**存在しない
     組み合わせ処方を新規に創作することはしない**、データベースに
     無ければ正直に「見つからなかった」旨を返し薬剤師・医師への相談を
     促す設計。
  3. **検証結果**: `cargo test --workspace --release`**32件全green**
     (既存28件+新規4件、regression無し)、`cargo clippy --workspace
     --all-targets --release -- -D warnings`警告0件。
  - 次にすべきこと: `cold_medicine_formulation`との統合(風邪薬配合案の
    構築時にこの推奨機能を自動的に呼び出す)、他の適応分類・処方の拡充。

- **2026-08-25(続き) 飲み合わせ注意喚起機能を追加(甘草の重複+ビタミン
  過剰摂取、ユーザー指示「漢方薬同士だと甘草の成分が重なると…注意喚起」
  「ビタミンBや栄養を摂取しすぎて…注意喚起機能」「基本は日本語と英語で」
  への対応)**:
  1. **事実確認と訂正(最重要、Google検索で調査・記録)**: ユーザーの
     説明「甘草の重複は肝臓に悪い」「ビタミンB過剰でがんになりやすい」
     は、公開されている医学・栄養学文献と照合したところ**不正確な点が
     あった**ため、正確な情報に訂正した上で実装した。
     - 甘草(グリチルリチン)の過剰摂取で実際に広く報告される主要リスクは
       **偽アルドステロン症**(腎臓でのコルチゾール分解酵素阻害による
       高血圧・低カリウム血症・四肢脱力・浮腫等)であり、肝毒性そのもの
       が主症状ではない(日本内分泌学会・厚生労働省の資料で確認、
       用量依存性のデータ〈1日1gで発症率1.0%、6gで11.1%〉も確認)。
     - ビタミンB1/B2/B6/B12は疫学メタアナリシスで腫瘍発生リスクとの
       関連が認められず、ナイアシン(B3)はむしろ負の相関が報告されて
       いる——「ビタミンB全般ががんの原因」という一般化は文献的支持が
       無い。一方**βカロテンのサプリメント高用量摂取(特に喫煙者)**は
       CARET/ATBC試験等の大規模臨床試験で肺がんリスク増加と明確に
       関連しており、これが実際に文献で裏付けられる該当リスクである。
  2. **実装内容**: `kampo_formula::glycyrrhiza_overlap_warning`
     (2処方以上の甘草含有処方併用を検出、日英併記の警告メッセージ)、
     `vitamin_supplement`モジュール新設(ナイアシン/B6/葉酸/プレフォーム
     ドビタミンA/サプリメントβカロテン/カフェインの耐容上限量[UL]+
     成分ごとに文献的に裏付けられる具体的なリスク説明を日英併記、
     複数摂取源[ビタミン剤+栄養ドリンク等]からの合計摂取量とULを
     比較する`check_cumulative_intake`)。
  3. **検証結果**: `cargo test --workspace --release`**28件全green**
     (既存20件+新規8件、regression無し)、`cargo clippy --workspace
     --all-targets --release -- -D warnings`警告0件。
  - 次にすべきこと: `cold_medicine_formulation`との統合(現状は独立した
    チェック関数、風邪薬配合案に漢方処方が複数含まれる場合の甘草重複
    チェックとの連携は未着手)、他の代表的な飲み合わせリスク(グレープ
    フルーツジュースと一部医薬品の相互作用等)への拡張は次回課題。

- **2026-08-25 リポジトリ新規作成、安全な範囲でのMVP実装(段階1)**:
  1. **スコープ確認(最重要)**: ユーザーの初期提案には「猛毒から毒性を
     取り除く機能」が含まれていたが、これは化学兵器・違法薬物の無毒化/
     精製手順の生成に転用され得るデュアルユースリスクがあるため実装
     できないと判断し、`AskUserQuestion`でユーザーに確認したところ
     「安全な部分のみ実装(化学式表示+OTC薬のDB+風邪薬シミュレータ)、
     毒性除去機能は実装しない」という明示的な合意を得た。この合意に
     基づき、以下の安全な範囲のみを実装した。
  2. **リポジトリ作成の経緯**: ローカル(`F:\runo`)・GitHub双方への
     新規リポジトリ作成はユーザーへの事前確認が必要な操作
     (メモリ`feedback_confirm_before_creating_folders`参照)のため、
     作成場所(`F:\runo\open-kagaku`)を`AskUserQuestion`で確認した上で
     着手。GitHub側リポジトリ本体は、この開発環境の`gh` CLIが
     未認証だったため(トークン設定なし)、ユーザー自身に
     `https://github.com/aon-co-jp/open-kagaku`を作成してもらい、
     それをcloneする形で進めた。
  3. **実装内容**: Rustワークスペース(`crates/kagaku-core`)、4モジュール:
     - `molecular_formula`: OTC医薬品有効成分名→分子式の参考データベース
       (10成分、アセトアミノフェン・イブプロフェン・アスピリン等)。
       **正直な開示**: 「あらゆる薬品名」を解決できる汎用パーサーでは
       なく限定的なリストのみ(将来PubChem等の公開APIへのオンライン
       照会に拡張する余地はあるが今回は未実装、外部ネットワーク依存を
       導入しない選択)。
     - `otc_ingredient`: OTC有効成分の薬効分類(解熱鎮痛薬/抗ヒスタミン薬/
       鎮咳薬/去痰薬/鼻閉改善薬/中枢神経刺激薬)+成人1日最大量の参考値
       (公開されている一般的な添付文書情報で広く引用される代表値、
       個別製品の正式な用法用量を代替しない旨を明記)。
     - `kampo_formula`: 漢方処方の参考DB(葛根湯・麻黄湯・小青竜湯・
       麦門冬湯の4処方、構成生薬+伝統的適応の一般的な説明)。**正直な
       開示**: 現代医学的なエビデンスを保証するものではない。
     - `cold_medicine_formulation`: 化学成分+漢方処方を組み合わせた
       「配合案」を構成し、同一薬効分類の重複(過量摂取リスクの初歩的な
       目安)を検出する一次スクリーニング。**正直な開示**: 実際の薬物
       相互作用・薬物動態計算・製剤化処方は一切行わない。
  4. **検証結果**: `cargo build --workspace --release`成功、
     `cargo test --workspace --release`**20件全green**(内訳:
     `molecular_formula`5件、`otc_ingredient`4件、`kampo_formula`4件、
     `cold_medicine_formulation`6件、警告0件)。`cargo clippy --workspace
     --all-targets --release -- -D warnings`警告0件。
  5. **見送った部分(正直な開示)**: (a) 毒性除去機能(上記1番の通り
     意図的に実装しない)、(b) HTTPサーバー・UI(今回はライブラリ
     クレートのみ)、(c) 東芝の擬似量子計算・DeepSeekの「折り畳み理論」・
     デジタルアニーラ・IOWN等の最新技術統合(性質の異なる技術を無理に
     混ぜると実体を伴わない「動くふり」になるリスクが高いと判断し、
     今回のMVPスコープでは着手していない——次回、個別に技術調査した
     上で本当に活用できる箇所があるか検討する)、(d) PubChem等外部
     APIへのオンライン化学式照会(現状は静的な内蔵リストのみ)。
  - 次にすべきこと: (1) `git init`・初回コミット・push(ユーザーの
    明示的な許可を得てから)、(2) open-cg-cadとの連携方法の検討
    (open-english⇄open-cg-cadと同様の相互リンクパターンを踏襲するか)、
    (3) HTTPサーバー+簡易UIの追加、(4) 有効成分・漢方処方DBの拡充、
    (5) 東芝/DeepSeek等の技術調査(個別に安全性・実現可能性を検討して
    から着手するかどうかを判断)。

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
