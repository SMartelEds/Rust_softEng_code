use revision_lab::aggregation::summarize;
use revision_lab::filtering::filter_valid;
use revision_lab::output::report;
use revision_lab::parsing::parse_many;
use revision_lab::step_log;

fn main() {
    let input = include_str!("../data/students.csv");

    step_log!("Parsing records");
    let records = parse_many(input);

    step_log!("Filtering records");
    let valid_records = filter_valid(records);

    step_log!("Creating summary");
    let summary = summarize(&valid_records);

    println!("{}", report(&summary));
}
