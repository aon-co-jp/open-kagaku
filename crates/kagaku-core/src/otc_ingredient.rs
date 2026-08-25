//! OTC(市販)医薬品有効成分の薬効分類・参考データベース。
//!
//! **正直な開示**: ここに収録する「成人1日最大量の目安」は、一般的な
//! 市販薬の添付文書・公開されている医薬品情報で広く引用される代表的な
//! 参考値であり、個別製品の正式な用法用量を代替しない。年齢・体重・
//! 既往症・他の服薬状況によって実際の上限は異なるため、必ず添付文書・
//! 薬剤師・医師の指示に従うこと。

/// 薬効分類。同じ分類の成分を複数同時に摂取すると、意図せず同じ作用が
/// 重複し過量摂取につながりやすい、という一次スクリーニングに用いる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrugClass {
    /// 解熱鎮痛薬。
    Analgesic,
    /// 抗ヒスタミン薬(抗アレルギー・鎮痒)。
    Antihistamine,
    /// 鎮咳薬。
    Antitussive,
    /// 去痰薬。
    Expectorant,
    /// 鼻閉改善薬(交感神経刺激薬)。
    Decongestant,
    /// 中枢神経刺激薬(眠気防止等の補助成分として配合されることがある)。
    Stimulant,
}

/// OTC医薬品有効成分。
#[derive(Debug, Clone, PartialEq)]
pub struct OtcIngredient {
    pub name: &'static str,
    pub drug_class: DrugClass,
    /// 成人1日最大量の目安(mg、参考値)。
    pub adult_max_daily_dose_mg_reference: f64,
}

pub fn known_ingredients() -> Vec<OtcIngredient> {
    vec![
        OtcIngredient { name: "acetaminophen", drug_class: DrugClass::Analgesic, adult_max_daily_dose_mg_reference: 4000.0 },
        OtcIngredient { name: "ibuprofen", drug_class: DrugClass::Analgesic, adult_max_daily_dose_mg_reference: 1200.0 },
        OtcIngredient { name: "chlorpheniramine", drug_class: DrugClass::Antihistamine, adult_max_daily_dose_mg_reference: 12.0 },
        OtcIngredient { name: "loratadine", drug_class: DrugClass::Antihistamine, adult_max_daily_dose_mg_reference: 10.0 },
        OtcIngredient { name: "dextromethorphan", drug_class: DrugClass::Antitussive, adult_max_daily_dose_mg_reference: 120.0 },
        OtcIngredient { name: "guaifenesin", drug_class: DrugClass::Expectorant, adult_max_daily_dose_mg_reference: 2400.0 },
        OtcIngredient { name: "pseudoephedrine", drug_class: DrugClass::Decongestant, adult_max_daily_dose_mg_reference: 240.0 },
        OtcIngredient { name: "caffeine", drug_class: DrugClass::Stimulant, adult_max_daily_dose_mg_reference: 400.0 },
    ]
}

pub fn find_by_name(name: &str) -> Option<OtcIngredient> {
    let normalized = name.trim().to_lowercase();
    known_ingredients().into_iter().find(|i| i.name == normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_by_name_is_case_insensitive() {
        assert!(find_by_name("Acetaminophen").is_some());
        assert!(find_by_name("ACETAMINOPHEN").is_some());
    }

    #[test]
    fn unknown_name_returns_none() {
        assert!(find_by_name("not-a-real-drug").is_none());
    }

    #[test]
    fn every_known_ingredient_has_a_positive_max_dose() {
        for i in known_ingredients() {
            assert!(i.adult_max_daily_dose_mg_reference > 0.0, "{} has non-positive max dose", i.name);
        }
    }

    #[test]
    fn two_analgesics_share_the_same_drug_class() {
        let a = find_by_name("acetaminophen").unwrap();
        let b = find_by_name("ibuprofen").unwrap();
        assert_eq!(a.drug_class, DrugClass::Analgesic);
        assert_eq!(a.drug_class, b.drug_class);
    }
}
