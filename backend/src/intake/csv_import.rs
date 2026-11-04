//! CSV intake: same fields as the JSON event, one event per row, with a header
//! row. Empty cells are treated as missing.

use crate::domain::models::PayoutEventInput;

/// Most rows accepted in one upload.
pub const MAX_ROWS: usize = 10_000;

/// Parse a CSV upload. Row errors are returned with their 1-based line number
/// (the header is line 1) instead of failing the whole file.
#[must_use]
pub fn parse(body: &[u8]) -> Vec<(usize, Result<PayoutEventInput, String>)> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(false)
        .from_reader(body);
    reader
        .deserialize::<PayoutEventInput>()
        .take(MAX_ROWS)
        .enumerate()
        .map(|(i, row)| (i + 2, row.map_err(|e| e.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state::PayoutState;

    const CSV: &str = "\
external_id,attempt,parent_external_id,state,reason,corridor,country,amount_minor,currency,stellar_tx_hash,memo,occurred_at
co_1,1,,processing,,USDC->NGN,NG,2500000,NGN,,co_1,2026-11-04T19:40:12Z
co_1,1,,completed,,USDC->NGN,NG,2500000,NGN,,co_1,2026-11-04T19:45:00Z
co_2,1,,teleported,,USDC->NGN,NG,100,NGN,,co_2,2026-11-04T19:45:00Z
";

    #[test]
    fn parses_rows_and_reports_bad_ones() {
        let rows = parse(CSV.as_bytes());
        assert_eq!(rows.len(), 3);
        let first = rows[0].1.as_ref().unwrap();
        assert_eq!(first.state, PayoutState::Processing);
        assert_eq!(first.parent_external_id, None);
        assert_eq!(first.stellar_tx_hash, None);
        assert_eq!(rows[1].0, 3);
        assert!(rows[2].1.is_err());
    }
}
