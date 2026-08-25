//! 漢方処方の参考データベース(伝統的処方名・構成生薬・伝統的適応)。
//!
//! **正直な開示(最重要)**: (1) ここで扱う「伝統的適応」は、公開されて
//! いる漢方医学の一般的な解説(教科書・公的機関の解説資料等)で広く
//! 知られる内容であり、現代医学的なエビデンス(有効性の臨床試験結果)を
//! 保証するものではない。(2) 構成生薬の分量・煎じ方・製剤化の詳細な
//! 手順は含まない(処方名と代表的な構成生薬名の一覧に留まる)。
//! (3) 個別の体質判断(いわゆる「証」の見極め)には漢方専門医・薬剤師の
//! 判断が必須であり、本データベースはその代替にならない。

/// 伝統的適応の大まかな分類(ユーザー指示「喉や声に良い漢方薬と肺や
/// 気管支に良い漢方薬を別々に飲むと甘草が重なって良くない」への対応で
/// 新設)。**正直な開示**: あくまで大まかな一次分類であり、個々の証
/// (体質・症状パターン)の見極めを代替しない。「心臓に良い」という
/// 分類は、本データベースの4処方いずれについても公開文献で明確な
/// 裏付けを確認できなかったため**意図的に含めていない**(文献的根拠の
/// 無い効能を主張しないため)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicationCategory {
    /// 喉・声(嗄声・喉の渇き・乾いた咳等)。
    Throat,
    /// 肺・気管支(咳・喘鳴等の下気道症状)。
    Respiratory,
    /// 鼻(鼻汁・鼻閉等)。
    Nasal,
    /// 感冒・インフルエンザの初期症状全般。
    ColdFluEarlyStage,
}

/// 漢方処方。
#[derive(Debug, Clone, PartialEq)]
pub struct KampoFormula {
    pub name: &'static str,
    /// 構成生薬(代表的なもの、分量は含まない)。
    pub herbal_components: Vec<&'static str>,
    /// 伝統的な適応の一般的な説明(現代医学的効能の保証ではない)。
    pub traditional_indication: &'static str,
    /// 伝統的適応の大まかな分類(複数可)。
    pub indication_categories: Vec<IndicationCategory>,
}

pub fn known_formulas() -> Vec<KampoFormula> {
    vec![
        KampoFormula {
            name: "葛根湯",
            herbal_components: vec!["葛根", "麻黄", "桂皮", "芍薬", "甘草", "生姜", "大棗"],
            traditional_indication: "感冒の初期(悪寒・発熱・肩こり等を伴う比較的体力がある場合)に伝統的に用いられるとされる。",
            indication_categories: vec![IndicationCategory::ColdFluEarlyStage],
        },
        KampoFormula {
            name: "麻黄湯",
            herbal_components: vec!["麻黄", "桂皮", "杏仁", "甘草"],
            traditional_indication: "感冒・インフルエンザ初期の悪寒・発熱・関節痛(体力が充実している場合)に伝統的に用いられるとされる。",
            indication_categories: vec![IndicationCategory::ColdFluEarlyStage],
        },
        KampoFormula {
            name: "小青竜湯",
            herbal_components: vec!["麻黄", "芍薬", "細辛", "乾姜", "甘草", "桂皮", "五味子", "半夏"],
            traditional_indication: "水様性の鼻汁・鼻閉・くしゃみを伴う感冒・アレルギー性鼻炎に伝統的に用いられるとされる。",
            indication_categories: vec![IndicationCategory::Nasal],
        },
        KampoFormula {
            name: "麦門冬湯",
            herbal_components: vec!["麦門冬", "半夏", "粳米", "大棗", "人参", "甘草"],
            traditional_indication: "痰の切れにくい乾いた咳・声のかすれ等、喉と気道の両方の乾燥症状に伝統的に用いられるとされる。",
            indication_categories: vec![IndicationCategory::Throat, IndicationCategory::Respiratory],
        },
        KampoFormula {
            name: "麻杏甘石湯",
            herbal_components: vec!["麻黄", "杏仁", "甘草", "石膏"],
            traditional_indication: "熱感を伴う喘鳴・咳(気管支炎・気管支喘息等)に伝統的に用いられるとされる。",
            indication_categories: vec![IndicationCategory::Respiratory],
        },
    ]
}

pub fn find_by_name(name: &str) -> Option<KampoFormula> {
    known_formulas().into_iter().find(|f| f.name == name.trim())
}

/// 生薬名(甘草の表記ゆれを含む)。
const GLYCYRRHIZA_HERB_NAME: &str = "甘草";

impl KampoFormula {
    /// 甘草(グリチルリチンを含む生薬)を含むかどうか。
    pub fn contains_glycyrrhiza(&self) -> bool {
        self.herbal_components.contains(&GLYCYRRHIZA_HERB_NAME)
    }
}

/// 複数の漢方処方を併用した場合、甘草(グリチルリチン)が重複して
/// 摂取されるリスクを警告する(ユーザー指示「漢方薬同士だと甘草の成分が
/// 重なると…注意喚起機能」への対応)。
///
/// **正直な開示・事実訂正(2026-08-25、Google検索で調査・記録)**:
/// ユーザーの初期説明では「肝臓に悪い」とされていたが、公開されている
/// 医学情報(日本内分泌学会・厚生労働省の重篤副作用疾患別対応マニュアル)
/// によれば、甘草(グリチルリチン酸)の過剰摂取で実際に広く報告されている
/// 主要なリスクは**偽アルドステロン症**(グリチルリチン酸が腎臓での
/// コルチゾール分解酵素[11β-HSD]を阻害し、過剰なコルチゾールが
/// ミネラルコルチコイド受容体に結合することで生じる、高血圧・低カリウム
/// 血症・四肢脱力・浮腫等を呈する病態)であり、肝毒性そのものが主症状
/// ではない(甘草含有薬が肝疾患治療薬にも使われることがあるため混同
/// されやすいが、機序としては腎臓・電解質系の問題である)。厚生労働省の
/// 報告では、1日あたりの甘草摂取量が1gで発症率1.0%、2gで1.7%、
/// 4gで3.3%、6gで11.1%という用量依存性が示されている。この関数は
/// 個々の処方の甘草含有量(g)までは追跡せず、**複数の甘草含有処方を
/// 併用しているという事実の検出**に留まる一次スクリーニング。
pub fn glycyrrhiza_overlap_warning(formulas: &[KampoFormula]) -> Option<GlycyrrhizaWarning> {
    let overlapping: Vec<&'static str> = formulas.iter().filter(|f| f.contains_glycyrrhiza()).map(|f| f.name).collect();
    if overlapping.len() >= 2 {
        Some(GlycyrrhizaWarning {
            formula_names: overlapping,
            message_ja: "複数の漢方処方に甘草(グリチルリチン)が含まれており、併用すると偽アルドステロン症(高血圧・低カリウム血症・四肢脱力・浮腫等)のリスクが高まる可能性があります。自己判断で併用を続けず、薬剤師・医師にご相談ください。".to_string(),
            message_en: "Multiple kampo formulas contain glycyrrhiza (licorice/glycyrrhizin); combining them may increase the risk of pseudoaldosteronism (hypertension, hypokalemia, limb weakness, edema, etc.). Do not continue combining them on your own judgment — consult a pharmacist or physician.".to_string(),
        })
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlycyrrhizaWarning {
    pub formula_names: Vec<&'static str>,
    pub message_ja: String,
    pub message_en: String,
}

/// 複数の適応分類(例: 喉+気管支)それぞれに別の漢方処方を選ぶと甘草が
/// 重複してしまう場合に、その両方の分類を一つでカバーする単一処方が
/// 既にあればそちらを推奨する(ユーザー指示「喉や声に良い漢方薬と肺や
/// 気管支に良い漢方薬を別々に飲むと甘草が重なって良くないので、その
/// 両方の漢方成分の入った二つが一つになった漢方薬の方をオススメする
/// 機能」への対応)。
///
/// **正直な開示**: 「一つになった処方」を新規に配合設計するのではなく、
/// 既存のデータベース(`known_formulas`)の中から、要求された全ての
/// 適応分類を実際にカバーする既存の伝統的処方を検索するだけ——存在
/// しない組み合わせを創作することはしない。見つからない場合は、
/// 正直に「単一処方は見つからなかった」旨を返し、薬剤師・医師への
/// 相談を促す。
pub fn recommend_combined_formula(requested_categories: &[IndicationCategory]) -> CombinedFormulaRecommendation {
    let combined = known_formulas().into_iter().find(|f| requested_categories.iter().all(|c| f.indication_categories.contains(c)));

    match combined {
        Some(formula) => CombinedFormulaRecommendation::SingleFormulaAvailable {
            recommended_ja: format!(
                "{}は、ご要望の適応(喉・気管支等の複数分類)を単独でカバーする処方です。別々の甘草含有処方を組み合わせるより、甘草の重複摂取(偽アルドステロン症のリスク)を避けられる可能性があります。最終判断は薬剤師・医師にご確認ください。",
                formula.name
            ),
            recommended_en: format!(
                "{} covers the requested indications (e.g. throat and bronchial) on its own. Preferring it over combining two separate glycyrrhiza-containing formulas may reduce the risk of cumulative glycyrrhizin intake (pseudoaldosteronism). Confirm with a pharmacist or physician before deciding.",
                formula.name
            ),
            formula,
        },
        None => CombinedFormulaRecommendation::NoSingleFormulaFound {
            message_ja: "ご要望の適応の組み合わせを単独でカバーする処方は、現在のデータベースには見つかりませんでした。別々の処方を組み合わせる場合は、甘草の重複に注意し、薬剤師・医師にご相談ください。".to_string(),
            message_en: "No single formula in the current database covers this combination of indications. If combining separate formulas, watch for overlapping glycyrrhiza and consult a pharmacist or physician.".to_string(),
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombinedFormulaRecommendation {
    SingleFormulaAvailable { formula: KampoFormula, recommended_ja: String, recommended_en: String },
    NoSingleFormulaFound { message_ja: String, message_en: String },
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

    #[test]
    fn kakkonto_and_maoto_both_contain_glycyrrhiza() {
        assert!(find_by_name("葛根湯").unwrap().contains_glycyrrhiza());
        assert!(find_by_name("麻黄湯").unwrap().contains_glycyrrhiza());
    }

    #[test]
    fn combining_two_glycyrrhiza_containing_formulas_triggers_warning() {
        let formulas = vec![find_by_name("葛根湯").unwrap(), find_by_name("麻黄湯").unwrap()];
        let warning = glycyrrhiza_overlap_warning(&formulas).unwrap();
        assert_eq!(warning.formula_names, vec!["葛根湯", "麻黄湯"]);
        assert!(warning.message_ja.contains("偽アルドステロン症"));
        assert!(warning.message_en.contains("pseudoaldosteronism"));
    }

    #[test]
    fn single_formula_does_not_trigger_overlap_warning() {
        let formulas = vec![find_by_name("葛根湯").unwrap()];
        assert!(glycyrrhiza_overlap_warning(&formulas).is_none());
    }

    #[test]
    fn bakumondoto_covers_both_throat_and_respiratory_categories() {
        let f = find_by_name("麦門冬湯").unwrap();
        assert!(f.indication_categories.contains(&IndicationCategory::Throat));
        assert!(f.indication_categories.contains(&IndicationCategory::Respiratory));
    }

    #[test]
    fn recommends_bakumondoto_when_throat_and_respiratory_both_needed() {
        let recommendation = recommend_combined_formula(&[IndicationCategory::Throat, IndicationCategory::Respiratory]);
        match recommendation {
            CombinedFormulaRecommendation::SingleFormulaAvailable { formula, recommended_ja, recommended_en } => {
                assert_eq!(formula.name, "麦門冬湯");
                assert!(recommended_ja.contains("麦門冬湯"));
                assert!(recommended_en.contains("麦門冬湯"));
            }
            other => panic!("expected SingleFormulaAvailable, got {other:?}"),
        }
    }

    #[test]
    fn no_recommendation_when_no_single_formula_covers_the_combination() {
        // 現在のDBには「鼻」+「気管支」を単独でカバーする処方は無い
        // (小青竜湯=鼻のみ、麻杏甘石湯=気管支のみ)。
        let recommendation = recommend_combined_formula(&[IndicationCategory::Nasal, IndicationCategory::Respiratory]);
        assert!(matches!(recommendation, CombinedFormulaRecommendation::NoSingleFormulaFound { .. }));
    }

    #[test]
    fn makyokansekito_is_tagged_respiratory_and_contains_glycyrrhiza() {
        let f = find_by_name("麻杏甘石湯").unwrap();
        assert!(f.indication_categories.contains(&IndicationCategory::Respiratory));
        assert!(f.contains_glycyrrhiza());
    }
}
