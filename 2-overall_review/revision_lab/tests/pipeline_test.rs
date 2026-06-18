use revision_lab::aggregation::summarize;
use revision_lab::filtering::filter_valid;
use revision_lab::parsing::{parse_line, parse_many};

#[test]
fn public_api_parses_line() {
    let record = parse_line("1,Alice,23,88.5,active").unwrap();
    assert!(record.is_active());
}

#[test]
fn pipeline_keeps_valid_records() {
    let input = "1,Alice,23,88.5,active\n2,Bob,,71.0,inactive\n3,Bad,200,90.0,active\nbad,line";

    let records = parse_many(input);
    let valid = filter_valid(records);
    let summary = summarize(&valid);

    assert_eq!(summary.total_count, 2);
    assert_eq!(summary.active_count, 1);
}
