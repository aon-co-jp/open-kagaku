//! open-kagakuの最小HTTP API+簡易UI(2026-08-25新設)。
//!
//! `kagaku-core`の各モジュール(化学式表示・OTC医薬品DB・漢方処方DB・
//! 風邪薬配合スクリーニング・飲み合わせ注意喚起・単一処方推奨)を
//! JSON APIとして公開し、`server/src/index.html`(`include_str!`で
//! 埋め込み)から呼び出す。`open-cg-cad/server`と同じ`RPoem`ベースの
//! 最小構成パターンを踏襲(車輪の再発明を避ける)。
//!
//! **⚠️ 最重要制約(必ず再掲)**: 本サーバーは医療機器・医薬品ソフト
//! ウェアではなく、教育・研究・設計検討用の参考ツールに過ぎない。
//! 実際の服薬判断・処方・製剤設計には薬剤師・医師等の有資格者による
//! レビューが必須であり、本サーバーの出力を根拠に服薬量を決定しては
//! ならない。

use kagaku_core::cold_medicine_formulation::{self, FormulationError};
use kagaku_core::kampo_formula::{self, CombinedFormulaRecommendation, IndicationCategory};
use kagaku_core::molecular_formula;
use kagaku_core::otc_ingredient;
use kagaku_core::vitamin_supplement::{self, VitaminOrMineral};
use open_runo_poem_compat::{get, handler_fn, post, Request, Response, Route, Server, StatusCode, TcpListener};
use std::net::SocketAddr;

fn rs_json_response(status: StatusCode, value: &impl serde::Serialize) -> Response {
    let body = serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec());
    hyper::Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(open_runo_poem_compat::hyper_compat::fixed_body(bytes::Bytes::from(body)))
        .expect("building a response from a fixed set of valid headers cannot fail")
}

fn html_response(body: &str) -> Response {
    hyper::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(open_runo_poem_compat::hyper_compat::fixed_body(bytes::Bytes::from(body.as_bytes().to_vec())))
        .expect("building a response from a fixed set of valid headers cannot fail")
}

async fn read_json_body<T: serde::de::DeserializeOwned>(req: Request) -> Result<T, Response> {
    use http_body_util::BodyExt;
    let bytes = match req.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return Err(rs_json_response(StatusCode::BAD_REQUEST, &serde_json::json!({"error": "failed to read request body"}))),
    };
    serde_json::from_slice::<T>(&bytes).map_err(|e| rs_json_response(StatusCode::BAD_REQUEST, &serde_json::json!({"error": format!("invalid JSON body: {e}")})))
}

async fn healthz() -> Response {
    rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true}))
}

#[derive(serde::Deserialize)]
struct MolecularFormulaRequest {
    ingredient_name: String,
}

async fn molecular_formula_lookup(req: Request) -> Response {
    let body: MolecularFormulaRequest = match read_json_body(req).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match molecular_formula::lookup_by_ingredient_name(&body.ingredient_name) {
        Some(formula) => rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "formula": formula.to_display_string()})),
        None => rs_json_response(
            StatusCode::NOT_FOUND,
            &serde_json::json!({"ok": false, "error": format!("no known molecular formula for '{}' (limited built-in list, not a universal lookup)", body.ingredient_name)}),
        ),
    }
}

async fn list_otc_ingredients() -> Response {
    let ingredients: Vec<_> = otc_ingredient::known_ingredients()
        .into_iter()
        .map(|i| serde_json::json!({"name": i.name, "drug_class": format!("{:?}", i.drug_class), "adult_max_daily_dose_mg_reference": i.adult_max_daily_dose_mg_reference}))
        .collect();
    rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "ingredients": ingredients}))
}

async fn list_kampo_formulas() -> Response {
    let formulas: Vec<_> = kampo_formula::known_formulas()
        .into_iter()
        .map(|f| {
            serde_json::json!({
                "name": f.name,
                "herbal_components": f.herbal_components,
                "traditional_indication": f.traditional_indication,
                "indication_categories": f.indication_categories.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>(),
                "contains_glycyrrhiza": f.contains_glycyrrhiza(),
            })
        })
        .collect();
    rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "formulas": formulas}))
}

#[derive(serde::Deserialize)]
struct FormulationRequest {
    chemical_ingredient_names: Vec<String>,
    kampo_formula_name: Option<String>,
}

async fn check_formulation(req: Request) -> Response {
    let body: FormulationRequest = match read_json_body(req).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match cold_medicine_formulation::build_formulation(&body.chemical_ingredient_names, body.kampo_formula_name.as_deref()) {
        Ok(result) => rs_json_response(
            StatusCode::OK,
            &serde_json::json!({
                "ok": true,
                "chemical_ingredients": result.chemical_ingredients.iter().map(|i| i.name).collect::<Vec<_>>(),
                "kampo_formula": result.kampo_formula.as_ref().map(|f| f.name),
                "duplicate_drug_class_warnings": result.duplicate_drug_class_warnings.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>(),
                "disclaimer_ja": result.disclaimer_ja,
            }),
        ),
        Err(e) => {
            let status = match e {
                FormulationError::EmptyFormulation => StatusCode::BAD_REQUEST,
                FormulationError::UnknownIngredient(_) | FormulationError::UnknownKampoFormula(_) => StatusCode::NOT_FOUND,
            };
            rs_json_response(status, &serde_json::json!({"ok": false, "error": e.to_string()}))
        }
    }
}

#[derive(serde::Deserialize)]
struct KampoOverlapRequest {
    formula_names: Vec<String>,
}

async fn check_kampo_overlap(req: Request) -> Response {
    let body: KampoOverlapRequest = match read_json_body(req).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let mut formulas = Vec::with_capacity(body.formula_names.len());
    for name in &body.formula_names {
        match kampo_formula::find_by_name(name) {
            Some(f) => formulas.push(f),
            None => return rs_json_response(StatusCode::NOT_FOUND, &serde_json::json!({"ok": false, "error": format!("unknown kampo formula: {name}")})),
        }
    }
    match kampo_formula::glycyrrhiza_overlap_warning(&formulas) {
        Some(warning) => rs_json_response(
            StatusCode::OK,
            &serde_json::json!({"ok": true, "warning": true, "formula_names": warning.formula_names, "message_ja": warning.message_ja, "message_en": warning.message_en}),
        ),
        None => rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "warning": false})),
    }
}

fn parse_indication_category(s: &str) -> Option<IndicationCategory> {
    match s {
        "throat" => Some(IndicationCategory::Throat),
        "respiratory" => Some(IndicationCategory::Respiratory),
        "nasal" => Some(IndicationCategory::Nasal),
        "cold_flu_early_stage" => Some(IndicationCategory::ColdFluEarlyStage),
        _ => None,
    }
}

#[derive(serde::Deserialize)]
struct RecommendRequest {
    categories: Vec<String>,
}

async fn recommend_kampo(req: Request) -> Response {
    let body: RecommendRequest = match read_json_body(req).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let mut categories = Vec::with_capacity(body.categories.len());
    for c in &body.categories {
        match parse_indication_category(c) {
            Some(cat) => categories.push(cat),
            None => return rs_json_response(StatusCode::BAD_REQUEST, &serde_json::json!({"ok": false, "error": format!("unknown category: {c} (expected one of: throat, respiratory, nasal, cold_flu_early_stage)")})),
        }
    }
    match kampo_formula::recommend_combined_formula(&categories) {
        CombinedFormulaRecommendation::SingleFormulaAvailable { formula, recommended_ja, recommended_en } => {
            rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "single_formula_available": true, "formula_name": formula.name, "message_ja": recommended_ja, "message_en": recommended_en}))
        }
        CombinedFormulaRecommendation::NoSingleFormulaFound { message_ja, message_en } => {
            rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "single_formula_available": false, "message_ja": message_ja, "message_en": message_en}))
        }
    }
}

fn parse_vitamin(s: &str) -> Option<VitaminOrMineral> {
    match s {
        "niacin_vitamin_b3" => Some(VitaminOrMineral::NiacinVitaminB3),
        "vitamin_b6" => Some(VitaminOrMineral::VitaminB6),
        "folic_acid" => Some(VitaminOrMineral::FolicAcid),
        "preformed_vitamin_a" => Some(VitaminOrMineral::PreformedVitaminA),
        "supplemental_beta_carotene" => Some(VitaminOrMineral::SupplementalBetaCarotene),
        "caffeine" => Some(VitaminOrMineral::Caffeine),
        _ => None,
    }
}

#[derive(serde::Deserialize)]
struct VitaminCheckRequest {
    item: String,
    intake_sources_mg: Vec<f64>,
}

async fn check_vitamin_intake(req: Request) -> Response {
    let body: VitaminCheckRequest = match read_json_body(req).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let item = match parse_vitamin(&body.item) {
        Some(v) => v,
        None => return rs_json_response(StatusCode::BAD_REQUEST, &serde_json::json!({"ok": false, "error": format!("unknown item: {}", body.item)})),
    };
    match vitamin_supplement::check_cumulative_intake(item, &body.intake_sources_mg) {
        Some(warning) => rs_json_response(
            StatusCode::OK,
            &serde_json::json!({
                "ok": true, "warning": true,
                "total_intake_mg": warning.total_intake_mg,
                "upper_limit_mg": warning.upper_limit_mg,
                "risk_note_ja": warning.risk_note_ja,
                "risk_note_en": warning.risk_note_en,
            }),
        ),
        None => rs_json_response(StatusCode::OK, &serde_json::json!({"ok": true, "warning": false})),
    }
}

const INDEX_HTML: &str = include_str!("index.html");

async fn index() -> Response {
    html_response(INDEX_HTML)
}

fn bind_addr() -> SocketAddr {
    std::env::var("OPEN_KAGAKU_SERVER_BIND").ok().and_then(|s| s.parse().ok()).unwrap_or_else(|| "127.0.0.1:4702".parse().unwrap())
}

#[tokio::main]
async fn main() {
    let addr = bind_addr();
    let app = Route::new()
        .at("/", get(handler_fn(move |_req, _p| async move { index().await })))
        .at("/healthz", get(handler_fn(move |_req, _p| async move { healthz().await })))
        .at("/v1/molecular-formula", post(handler_fn(move |req, _p| async move { molecular_formula_lookup(req).await })))
        .at("/v1/otc-ingredients", get(handler_fn(move |_req, _p| async move { list_otc_ingredients().await })))
        .at("/v1/kampo-formulas", get(handler_fn(move |_req, _p| async move { list_kampo_formulas().await })))
        .at("/v1/formulation/check", post(handler_fn(move |req, _p| async move { check_formulation(req).await })))
        .at("/v1/kampo/overlap-check", post(handler_fn(move |req, _p| async move { check_kampo_overlap(req).await })))
        .at("/v1/kampo/recommend", post(handler_fn(move |req, _p| async move { recommend_kampo(req).await })))
        .at("/v1/vitamin/check", post(handler_fn(move |req, _p| async move { check_vitamin_intake(req).await })));

    println!("open-kagaku server listening on http://{addr}/");
    let (bound_addr, handle) = Server::new(TcpListener::bind(addr)).run(app).await.expect("failed to bind local server (is the port already in use?)");
    println!("bound to http://{bound_addr}/");
    handle.await.expect("server task panicked");
}
