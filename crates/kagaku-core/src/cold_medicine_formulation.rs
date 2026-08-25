//! 風邪薬(漢方+化学薬品ハイブリッド)配合の一次スクリーニングツール
//! (ユーザー指示「風邪薬は漢方薬と化学薬品のハイブリッドなコフト顆粒の
//! 様な風邪薬も作れる薬品シミュレーター」への対応)。
//!
//! **⚠️ 正直な開示(最重要、他モジュールと同様に省略・弱化しない)**:
//! 本モジュールが行うのは「同じ薬効分類の化学成分が重複していないか」
//! という最も基本的な一次スクリーニングのみであり、(1) 実際の薬物
//! 相互作用(代謝酵素競合・血中濃度への影響等)の計算は一切行わない、
//! (2) 漢方処方と化学成分の相互作用(いわゆる「相性」)の評価も行わない、
//! (3) 実際の製剤化(顆粒・錠剤等への加工)の処方箋を生成するものでは
//! ない。**本ツールの出力を根拠に実際の配合・服薬量を決定してはならず、
//! 薬剤師・医師による確認が必須**。

use crate::kampo_formula::{self, KampoFormula};
use crate::otc_ingredient::{self, DrugClass, OtcIngredient};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FormulationError {
    #[error("unknown OTC ingredient: {0}")]
    UnknownIngredient(String),
    #[error("unknown kampo formula: {0}")]
    UnknownKampoFormula(String),
    #[error("at least one chemical ingredient or a kampo formula must be specified")]
    EmptyFormulation,
}

/// 配合結果。
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFormulation {
    pub chemical_ingredients: Vec<OtcIngredient>,
    pub kampo_formula: Option<KampoFormula>,
    /// 2種類以上の成分が同じ薬効分類に属する場合、その分類の一覧
    /// (過量摂取リスクの初歩的な目安として一次スクリーニングで検出)。
    pub duplicate_drug_class_warnings: Vec<DrugClass>,
    pub disclaimer_ja: &'static str,
}

const DISCLAIMER_JA: &str = "これは薬効分類の重複を検出する一次スクリーニングに過ぎません。実際の薬物相互作用・製剤化・服薬量の決定には薬剤師・医師の確認が必須です。";

/// 化学成分名のリスト(空でも可)と、任意の漢方処方名から、ハイブリッド
/// 風邪薬の配合案を構成する。
pub fn build_formulation(chemical_ingredient_names: &[String], kampo_formula_name: Option<&str>) -> Result<ResolvedFormulation, FormulationError> {
    if chemical_ingredient_names.is_empty() && kampo_formula_name.is_none() {
        return Err(FormulationError::EmptyFormulation);
    }

    let mut chemical_ingredients = Vec::with_capacity(chemical_ingredient_names.len());
    for name in chemical_ingredient_names {
        let ingredient = otc_ingredient::find_by_name(name).ok_or_else(|| FormulationError::UnknownIngredient(name.clone()))?;
        chemical_ingredients.push(ingredient);
    }

    let kampo = match kampo_formula_name {
        Some(name) => Some(kampo_formula::find_by_name(name).ok_or_else(|| FormulationError::UnknownKampoFormula(name.to_string()))?),
        None => None,
    };

    let mut class_counts: std::collections::HashMap<DrugClass, u32> = std::collections::HashMap::new();
    for ingredient in &chemical_ingredients {
        *class_counts.entry(ingredient.drug_class).or_insert(0) += 1;
    }
    let mut duplicate_drug_class_warnings: Vec<DrugClass> = class_counts.into_iter().filter(|(_, count)| *count >= 2).map(|(class, _)| class).collect();
    // 決定的な出力順にするため(HashMapのイテレーション順は不定)。
    duplicate_drug_class_warnings.sort_by_key(|c| format!("{c:?}"));

    Ok(ResolvedFormulation { chemical_ingredients, kampo_formula: kampo, duplicate_drug_class_warnings, disclaimer_ja: DISCLAIMER_JA })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typical_hybrid_cold_medicine_has_no_duplicate_class_warning() {
        let names = vec!["acetaminophen".to_string(), "dextromethorphan".to_string(), "guaifenesin".to_string()];
        let result = build_formulation(&names, Some("葛根湯")).unwrap();
        assert_eq!(result.chemical_ingredients.len(), 3);
        assert!(result.kampo_formula.is_some());
        assert!(result.duplicate_drug_class_warnings.is_empty());
    }

    #[test]
    fn two_analgesics_trigger_a_duplicate_class_warning() {
        let names = vec!["acetaminophen".to_string(), "ibuprofen".to_string()];
        let result = build_formulation(&names, None).unwrap();
        assert_eq!(result.duplicate_drug_class_warnings, vec![DrugClass::Analgesic]);
    }

    #[test]
    fn unknown_chemical_ingredient_is_rejected_honestly() {
        let names = vec!["not-a-real-ingredient".to_string()];
        assert_eq!(build_formulation(&names, None).unwrap_err(), FormulationError::UnknownIngredient("not-a-real-ingredient".to_string()));
    }

    #[test]
    fn unknown_kampo_formula_is_rejected_honestly() {
        let names = vec!["acetaminophen".to_string()];
        assert_eq!(build_formulation(&names, Some("存在しない処方")).unwrap_err(), FormulationError::UnknownKampoFormula("存在しない処方".to_string()));
    }

    #[test]
    fn empty_formulation_is_rejected() {
        assert_eq!(build_formulation(&[], None).unwrap_err(), FormulationError::EmptyFormulation);
    }

    #[test]
    fn kampo_only_formulation_is_allowed() {
        let result = build_formulation(&[], Some("麻黄湯")).unwrap();
        assert!(result.chemical_ingredients.is_empty());
        assert!(result.kampo_formula.is_some());
    }

    #[test]
    fn disclaimer_is_always_present() {
        let result = build_formulation(&["acetaminophen".to_string()], None).unwrap();
        assert!(!result.disclaimer_ja.is_empty());
    }
}
