//! 医薬品有効成分名から分子式を引く、教育目的の参考データベース。
//!
//! **正直な開示(最重要)**: (1) ここに収録されているのは一般的な
//! OTC医薬品有効成分のうち、公開されている化学情報(教科書・薬局方等)
//! で広く知られる分子式のみの**限定的な**リストであり、「あらゆる薬品名」
//! を解決できる汎用的な化学命名法パーサーではない(将来、PubChem等の
//! 公開APIへのオンライン照会に拡張する余地はあるが、本バージョンでは
//! 未実装)。(2) 合成経路・製剤処方・精製手順は一切含まない——分子式
//! (原子の種類と個数)という最も基本的な化学情報の参照に留まる。

use std::collections::HashMap;

/// 分子式(原子記号→個数のマップとして表現。表示用の文字列組み立ては
/// `to_display_string`で行う)。
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularFormula {
    pub atom_counts: Vec<(String, u32)>,
}

impl MolecularFormula {
    pub fn new(atoms: &[(&str, u32)]) -> Self {
        MolecularFormula { atom_counts: atoms.iter().map(|(s, n)| (s.to_string(), *n)).collect() }
    }

    /// 一般的な化学式の表記順(炭素→水素→その他をアルファベット順)に
    /// 従った表示用文字列(例: "C9H8O4")を組み立てる。
    pub fn to_display_string(&self) -> String {
        let mut atoms = self.atom_counts.clone();
        atoms.sort_by_key(|(symbol, _)| match symbol.as_str() {
            "C" => (0u8, String::new()),
            "H" => (1u8, String::new()),
            other => (2u8, other.to_string()),
        });
        let mut out = String::new();
        for (symbol, count) in atoms {
            out.push_str(&symbol);
            if count != 1 {
                out.push_str(&count.to_string());
            }
        }
        out
    }
}

/// 医薬品有効成分名(小文字・スペース無し正規化前提)から分子式を引く。
pub fn lookup_by_ingredient_name(name: &str) -> Option<MolecularFormula> {
    known_formulas().get(&normalize(name)).cloned()
}

fn normalize(name: &str) -> String {
    name.trim().to_lowercase().replace([' ', '-', '_'], "")
}

fn known_formulas() -> HashMap<String, MolecularFormula> {
    let entries: Vec<(&str, MolecularFormula)> = vec![
        ("acetaminophen", MolecularFormula::new(&[("C", 8), ("H", 9), ("N", 1), ("O", 2)])),
        ("paracetamol", MolecularFormula::new(&[("C", 8), ("H", 9), ("N", 1), ("O", 2)])),
        ("ibuprofen", MolecularFormula::new(&[("C", 13), ("H", 18), ("O", 2)])),
        ("aspirin", MolecularFormula::new(&[("C", 9), ("H", 8), ("O", 4)])),
        ("dextromethorphan", MolecularFormula::new(&[("C", 18), ("H", 25), ("N", 1), ("O", 1)])),
        ("chlorpheniramine", MolecularFormula::new(&[("C", 16), ("H", 19), ("Cl", 1), ("N", 2)])),
        ("guaifenesin", MolecularFormula::new(&[("C", 10), ("H", 14), ("O", 4)])),
        ("pseudoephedrine", MolecularFormula::new(&[("C", 10), ("H", 15), ("N", 1), ("O", 1)])),
        ("loratadine", MolecularFormula::new(&[("C", 22), ("H", 23), ("Cl", 1), ("N", 2), ("O", 2)])),
        ("caffeine", MolecularFormula::new(&[("C", 8), ("H", 10), ("N", 4), ("O", 2)])),
    ];
    entries.into_iter().map(|(name, formula)| (normalize(name), formula)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_known_ingredient_case_and_space_insensitively() {
        let f1 = lookup_by_ingredient_name("Acetaminophen").unwrap();
        let f2 = lookup_by_ingredient_name(" acetaminophen ").unwrap();
        assert_eq!(f1, f2);
        assert_eq!(f1.to_display_string(), "C8H9NO2");
    }

    #[test]
    fn unknown_ingredient_returns_none_honestly() {
        assert_eq!(lookup_by_ingredient_name("totally-made-up-chemical-xyz"), None);
    }

    #[test]
    fn display_string_orders_carbon_then_hydrogen_then_alphabetical() {
        let f = MolecularFormula::new(&[("O", 2), ("N", 1), ("H", 9), ("C", 8)]);
        assert_eq!(f.to_display_string(), "C8H9NO2");
    }

    #[test]
    fn single_atom_count_is_omitted_in_display() {
        let f = MolecularFormula::new(&[("C", 1), ("H", 4)]);
        assert_eq!(f.to_display_string(), "CH4");
    }

    #[test]
    fn all_known_ingredients_resolve_to_a_nonempty_formula() {
        for name in ["ibuprofen", "aspirin", "dextromethorphan", "chlorpheniramine", "guaifenesin", "pseudoephedrine", "loratadine", "caffeine"] {
            let f = lookup_by_ingredient_name(name).unwrap_or_else(|| panic!("expected formula for {name}"));
            assert!(!f.atom_counts.is_empty());
        }
    }
}
