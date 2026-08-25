//! ビタミン剤・栄養ドリンクの過剰摂取に対する注意喚起(ユーザー指示
//! 「ビタミン剤や栄養ドリンクなどは、ビタミンBや栄養を摂取しすぎて、
//! 一日の最大摂取量を大幅に超えて摂取するとガンになりやすい傾向など
//! 注意喚起機能」への対応)。
//!
//! **⚠️ 正直な開示・事実訂正(最重要、2026-08-25にGoogle検索で調査・
//! 記録)**: ユーザーの初期説明は「ビタミンB全般の過剰摂取ががんに
//! つながる」という一般化だったが、公開されている栄養学の文献
//! (EFSA・NIH等の耐容上限量[UL]設定資料、複数の疫学メタアナリシス)を
//! 確認したところ、**この一般化は正確ではない**:
//! - ビタミンB1・B2・B6・B12は、あるメタアナリシスでは腫瘍発生リスクとの
//!   関連が認められなかった。
//! - ナイアシン(ビタミンB3)は、同分析でむしろ腫瘍発生率と**負の相関**
//!   (摂取量が多いほどリスクが低い側)が報告されている——「ビタミンBが
//!   がんを引き起こす」の根拠にはならない。ただしナイアシンは高用量で
//!   肝毒性・顔面紅潮等の急性副作用が知られており、耐容上限量(成人
//!   35mg/日)は別の理由(がんではなく肝機能障害等)で設定されている。
//! - **実際にがんリスクとの関連が最も明確に文献で示されているのは
//!   βカロテン(サプリメントとしての高用量摂取)とプレフォームド
//!   ビタミンA**であり、特に喫煙者においてβカロテンのサプリメント
//!   高用量摂取(1日20mg以上)は肺がんリスク増加と関連することが
//!   大規模臨床試験(CARET・ATBC試験等)で示されている。
//! - ビタミンB6は高用量・長期摂取で末梢神経障害(感覚異常等)のリスクが
//!   知られている(がんとは別の副作用)。
//!
//! 本モジュールは、ユーザーの意図(過剰摂取への注意喚起)は汲みつつ、
//! **実際に文献で裏付けられる具体的なリスクをビタミン/成分ごとに
//! 正直に記載する**方針を取り、「ビタミンBは全てがんの原因」という
//! 不正確な一般化はしない。

/// ビタミン・ミネラル・関連成分。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VitaminOrMineral {
    NiacinVitaminB3,
    VitaminB6,
    FolicAcid,
    PreformedVitaminA,
    /// サプリメントとしてのβカロテン(食品由来の摂取とは区別)。
    SupplementalBetaCarotene,
    Caffeine,
}

/// 耐容上限量(UL)と、実際に文献で裏付けられる過剰摂取リスクの説明
/// (日英併記、ユーザー指示「基本は日本語と英語で」への対応)。
#[derive(Debug, Clone, PartialEq)]
pub struct VitaminReference {
    pub name_ja: &'static str,
    pub name_en: &'static str,
    /// 成人の耐容上限量の目安(mg/日)。
    pub adult_upper_limit_mg_reference: f64,
    pub overconsumption_risk_ja: &'static str,
    pub overconsumption_risk_en: &'static str,
}

pub fn reference_for(item: VitaminOrMineral) -> VitaminReference {
    use VitaminOrMineral::*;
    match item {
        NiacinVitaminB3 => VitaminReference {
            name_ja: "ナイアシン(ビタミンB3)",
            name_en: "Niacin (Vitamin B3)",
            adult_upper_limit_mg_reference: 35.0,
            overconsumption_risk_ja: "高用量で顔面紅潮・肝機能障害等の急性副作用が知られている(がんリスクとの関連は文献上むしろ否定的)。",
            overconsumption_risk_en: "High doses are known to cause flushing and liver function abnormalities (evidence does not support a cancer-risk link; some studies suggest the opposite association).",
        },
        VitaminB6 => VitaminReference {
            name_ja: "ビタミンB6",
            name_en: "Vitamin B6",
            adult_upper_limit_mg_reference: 100.0,
            overconsumption_risk_ja: "高用量・長期摂取で末梢神経障害(感覚異常等)のリスクが知られている(がんリスクとの明確な関連は確立していない)。",
            overconsumption_risk_en: "High-dose, long-term intake is known to risk peripheral neuropathy (sensory disturbances); a clear cancer-risk link is not established.",
        },
        FolicAcid => VitaminReference {
            name_ja: "葉酸",
            name_en: "Folic acid",
            adult_upper_limit_mg_reference: 1.0,
            overconsumption_risk_ja: "高用量摂取はビタミンB12欠乏の症状(貧血等)を覆い隠すおそれがあるとされる。",
            overconsumption_risk_en: "High intake may mask symptoms of vitamin B12 deficiency (such as anemia).",
        },
        PreformedVitaminA => VitaminReference {
            name_ja: "プレフォームドビタミンA(レチノール)",
            name_en: "Preformed Vitamin A (retinol)",
            adult_upper_limit_mg_reference: 3.0,
            overconsumption_risk_ja: "過剰摂取は肝毒性のほか、疫学研究で腫瘍発生率との正の相関が報告されている。",
            overconsumption_risk_en: "Excess intake is linked to liver toxicity, and epidemiological studies report a positive correlation with tumor incidence.",
        },
        SupplementalBetaCarotene => VitaminReference {
            name_ja: "βカロテン(サプリメント由来)",
            name_en: "Beta-carotene (supplemental)",
            adult_upper_limit_mg_reference: 7.0,
            overconsumption_risk_ja: "特に喫煙者において、サプリメントとしての高用量摂取(1日20mg以上)は大規模臨床試験(CARET・ATBC試験等)で肺がんリスク増加と関連することが示されている。食品由来の摂取とは区別される。",
            overconsumption_risk_en: "Especially in smokers, high-dose supplemental intake (20 mg/day or more) has been linked to increased lung cancer risk in large clinical trials (CARET, ATBC). This is distinct from dietary intake.",
        },
        Caffeine => VitaminReference {
            name_ja: "カフェイン(栄養ドリンク等に含まれる)",
            name_en: "Caffeine (found in energy drinks, etc.)",
            adult_upper_limit_mg_reference: 400.0,
            overconsumption_risk_ja: "過剰摂取は動悸・不眠・カフェイン中毒(重篤な場合は不整脈等)のリスクがある(がんリスクとの確立した関連は無い)。",
            overconsumption_risk_en: "Overconsumption risks palpitations, insomnia, and caffeine intoxication (arrhythmia in severe cases); no established cancer-risk link.",
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverconsumptionWarning {
    pub item: VitaminOrMineral,
    pub total_intake_mg: f64,
    pub upper_limit_mg: f64,
    pub risk_note_ja: &'static str,
    pub risk_note_en: &'static str,
}

/// 複数の摂取源(ビタミン剤+栄養ドリンク等)からの同一成分の合計摂取量を
/// 耐容上限量(UL)と比較し、超過している場合に警告を返す。
///
/// **正直な開示**: 上限を超えたからといって必ず有害事象が起きることを
/// 保証するものではなく(個人差がある)、逆に上限未満でも安全性を保証
/// するものでもない一次的な目安に過ぎない。
pub fn check_cumulative_intake(item: VitaminOrMineral, intake_sources_mg: &[f64]) -> Option<OverconsumptionWarning> {
    let total: f64 = intake_sources_mg.iter().sum();
    let reference = reference_for(item);
    if total > reference.adult_upper_limit_mg_reference {
        Some(OverconsumptionWarning {
            item,
            total_intake_mg: total,
            upper_limit_mg: reference.adult_upper_limit_mg_reference,
            risk_note_ja: reference.overconsumption_risk_ja,
            risk_note_en: reference.overconsumption_risk_en,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intake_within_limit_triggers_no_warning() {
        assert!(check_cumulative_intake(VitaminOrMineral::NiacinVitaminB3, &[10.0, 5.0]).is_none());
    }

    #[test]
    fn cumulative_intake_from_multiple_sources_exceeding_limit_triggers_warning() {
        // ビタミン剤20mg + 栄養ドリンク20mg = 40mg > UL(35mg)。
        let warning = check_cumulative_intake(VitaminOrMineral::NiacinVitaminB3, &[20.0, 20.0]).unwrap();
        assert!((warning.total_intake_mg - 40.0).abs() < 1e-9);
        assert_eq!(warning.upper_limit_mg, 35.0);
    }

    #[test]
    fn beta_carotene_warning_mentions_lung_cancer_risk_for_smokers_not_general_b_vitamins() {
        let warning = check_cumulative_intake(VitaminOrMineral::SupplementalBetaCarotene, &[10.0]).unwrap();
        assert!(warning.risk_note_ja.contains("肺がん"));
        assert!(warning.risk_note_en.contains("lung cancer"));
    }

    #[test]
    fn niacin_risk_note_does_not_overclaim_a_cancer_link() {
        let reference = reference_for(VitaminOrMineral::NiacinVitaminB3);
        assert!(reference.overconsumption_risk_ja.contains("否定的") || reference.overconsumption_risk_ja.contains("肝機能"));
    }

    #[test]
    fn every_reference_has_bilingual_nonempty_text() {
        for item in [
            VitaminOrMineral::NiacinVitaminB3,
            VitaminOrMineral::VitaminB6,
            VitaminOrMineral::FolicAcid,
            VitaminOrMineral::PreformedVitaminA,
            VitaminOrMineral::SupplementalBetaCarotene,
            VitaminOrMineral::Caffeine,
        ] {
            let r = reference_for(item);
            assert!(!r.name_ja.is_empty() && !r.name_en.is_empty());
            assert!(!r.overconsumption_risk_ja.is_empty() && !r.overconsumption_risk_en.is_empty());
            assert!(r.adult_upper_limit_mg_reference > 0.0);
        }
    }
}
