use chrono::{DateTime, Duration, Utc};
use crate::model::timeseries::LoadEntry;
use crate::model::srl::SRLEntry;

/// Generates a timestamp vector at fixed minute intervals between start and end.
/// Example: generate_time_grid(t0, t1, 1) → [t0, t0+1min, t0+2min, ..., t1]
pub fn generate_time_grid(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    step_minutes: i64,
) -> Vec<DateTime<Utc>> {
    let mut times = Vec::new();
    let mut current = start;

    while current <= end {
        times.push(current);
        current = current + Duration::minutes(step_minutes);
    }

    times
}


/// Linearly interpolates a scalar value between two timestamps.
/// alpha = (target - t0) / (t1 - t0)
pub fn interpolate_scalar(
    t0: DateTime<Utc>,
    t1: DateTime<Utc>,
    v0: f64,
    v1: f64,
    target: DateTime<Utc>,
) -> f64 {
    let dt = (t1 - t0).num_milliseconds() as f64;
    let dt_target = (target - t0).num_milliseconds() as f64;

    if dt.abs() < 1e-9 {
        return v0;
    }

    let alpha = dt_target / dt;
    v0 + alpha * (v1 - v0)
}


/// Interpolates a full LoadEntry series (power_kw) to 1-min resolution.
pub fn interpolate_load_to_1min(
    input: &[LoadEntry],
    target_timestamps: &[DateTime<Utc>],
) -> Vec<LoadEntry> {
    let mut result = Vec::with_capacity(target_timestamps.len());

    for &ts in target_timestamps {
        let mut prev: Option<&LoadEntry> = None;
        let mut next: Option<&LoadEntry> = None;

        for point in input {
            if point.timestamp <= ts {
                prev = Some(point);
            } else {
                next = Some(point);
                break;
            }
        }

        let power_kw = match (prev, next) {
            (Some(p0), Some(p1)) => interpolate_scalar(
                p0.timestamp, p1.timestamp,
                p0.power_kw, p1.power_kw,
                ts
            ),
            _ => 0.0,
        };

        result.push(LoadEntry { timestamp: ts, power_kw });
    }

    result
}


/// Interpolates a full SRLEntry series (energy + price) to 1-min resolution.
pub fn interpolate_srl_to_1min(
    input: &[SRLEntry],
    target_timestamps: &[DateTime<Utc>],
) -> Vec<SRLEntry> {
    let mut result = Vec::with_capacity(target_timestamps.len());

    for &ts in target_timestamps {
        let mut prev: Option<&SRLEntry> = None;
        let mut next: Option<&SRLEntry> = None;

        for point in input {
            if point.timestamp <= ts {
                prev = Some(point);
            } else {
                next = Some(point);
                break;
            }
        }

        let (pos_kwh, neg_kwh, pos_price, neg_price) = match (prev, next) {
            (Some(p0), Some(p1)) => (
                interpolate_scalar(p0.timestamp, p1.timestamp, p0.pos_energy_kwh, p1.pos_energy_kwh, ts),
                interpolate_scalar(p0.timestamp, p1.timestamp, p0.neg_energy_kwh, p1.neg_energy_kwh, ts),
                interpolate_scalar(p0.timestamp, p1.timestamp, p0.pos_price_eur_mwh, p1.pos_price_eur_mwh, ts),
                interpolate_scalar(p0.timestamp, p1.timestamp, p0.neg_price_eur_mwh, p1.neg_price_eur_mwh, ts),
            ),
            _ => (0.0, 0.0, 0.0, 0.0),
        };

        result.push(SRLEntry {
            timestamp: ts,
            pos_energy_kwh: pos_kwh,
            neg_energy_kwh: neg_kwh,
            pos_price_eur_mwh: pos_price,
            neg_price_eur_mwh: neg_price,
        });
    }

    result
}



