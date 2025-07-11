use crate::model::timeseries::LoadEntry;
use crate::model::srl::SRLEntry;
use crate::model::mergedseries::MergedTick;


pub fn merge_1min_series(
    load: &[LoadEntry],
    srl: &[SRLEntry],
) -> Vec<MergedTick> {
    load.iter()
        .zip(srl.iter())
        .map(|(l, s)| MergedTick {
            timestamp: l.timestamp,
            power_kw: l.power_kw,
            srl_pos_kwh: s.pos_energy_kwh,
            srl_neg_kwh: s.neg_energy_kwh,
            srl_pos_price_eur_mwh: s.pos_price_eur_mwh,
            srl_neg_price_eur_mwh: s.neg_price_eur_mwh,
        })
        .collect()
}
