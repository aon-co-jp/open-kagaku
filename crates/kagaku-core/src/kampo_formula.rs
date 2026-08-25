//! 漢方処方の参考データベース(伝統的処方名・構成生薬・伝統的適応)。
//!
//! **正直な開示(最重要)**: (1) ここで扱う「伝統的適応」は、公開されて
//! いる漢方医学の一般的な解説(教科書・公的機関の解説資料等)で広く
//! 知られる内容であり、現代医学的なエビデンス(有効性の臨床試験結果)を
//! 保証するものではない。(2) 構成生薬の分量・煎じ方・製剤化の詳細な
//! 手順は含まない(処方名と代表的な構成生薬名の一覧に留まる)。
//! (3) 個別の体質判断(いわゆる「証」の見極め)には漢方専門医・薬剤師の
//! 判断が必須であり、本データベースはその代替にならない。

/// 漢方処方。
#[derive(Debug, Clone, PartialEq)]
pub struct KampoFormula {
    pub name: &'static str,
    /// 構成生薬(代表的なもの、分量は含まない)。
    pub herbal_components: Vec<&'static str>,
    /// 伝統的な適応の一般的な説明(現代医学的効能の保証ではない)。
    pub traditional_indication: &'static str,
}

pub fn known_formulas() -> Vec<KampoFormula> {
    vec![
        KampoFormula {
            name: "葛根湯",
            herbal_components: vec!["葛根", "麻黄", "桂皮", "芍薬", "甘草", "生姜", "大棗"],
            traditional_indication: "感冒の初期(悪寒・発熱・肩こり等を伴う比較的体力がある場合)に伝統的に用いられるとされる。",
        },
        KampoFormula {
            name: "麻黄湯",
            herbal_components: vec!["麻黄", "桂皮", "杏仁", "甘草"],
            traditional_indication: "感冒・インフルエンザ初期の悪寒・発熱・関節痛(体力が充実している場合)に伝統的に用いられるとされる。",
        },
        KampoFormula {
            name: "小青竜湯",
            herbal_components: vec!["麻黄", "芍薬", "細辛", "乾姜", "甘草", "桂皮", "五味子", "半夏"],
            traditional_indication: "水様性の鼻汁・鼻閉・くしゃみを伴う感冒・アレルギー性鼻炎に伝統的に用いられるとされる。",
        },
        KampoFormula {
            name: "麦門冬湯",
            herbal_components: vec!["麦門冬", "半夏", "粳米", "大棗", "人参", "甘草"],
            traditional_indication: "痰の切れにくい乾いた咳に伝統的に用いられるとされる。",
        },
    ]
}

pub fn find_by_name(name: &str) -> Option<KampoFormula> {
    known_formulas().into_iter().find(|f| f.name == name.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_by_name_finds_kakkonto() {
        let f = find_by_name("葛根湯").unwrap();
        assert!(f.herbal_components.contains(&"葛根"));
    }

    #[test]
    fn unknown_formula_name_returns_none() {
        assert!(find_by_name("存在しない処方名").is_none());
    }

    #[test]
    fn every_known_formula_has_at_least_one_herbal_component() {
        for f in known_formulas() {
            assert!(!f.herbal_components.is_empty(), "{} has no herbal components", f.name);
        }
    }

    #[test]
    fn formula_names_are_unique() {
        let formulas = known_formulas();
        for i in 0..formulas.len() {
            for j in (i + 1)..formulas.len() {
                assert_ne!(formulas[i].name, formulas[j].name);
            }
        }
    }
}
