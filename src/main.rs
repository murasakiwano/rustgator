use std::collections::BTreeMap;
use std::{error::Error, path::PathBuf};

use clap::Parser;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color};
use csv::Reader;
use regex::RegexSetBuilder;

#[derive(Parser)]
#[command(
    name = "rustgator",
    version,
    about = "read and aggregate CSV files",
    author = "Johann Homonnai <github.com/murasakiwano>"
)]
struct Cli {
    /// Path to the csv file
    file: PathBuf,

    /// Which header to group the records by
    group_by: String,

    /// If this flag is present, will group the amounts which match a regex
    #[arg(short, long)]
    pattern: Option<Vec<String>>,

    /// Whether the matching will be case insensitive (defaults to false)
    #[arg(short = 'i', long)]
    case_insensitive: bool,

    /// If negative values should be removed
    #[arg(long)]
    remove_negatives: bool,

    /// If positive values should be removed
    #[arg(long)]
    remove_positives: bool,

    /// Optional flag to set the column which should be calculated. Defaults to `amount`
    #[arg(short, long = "column", default_value_t = String::from("amount"))]
    column_to_calculate: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let file = std::fs::File::open(cli.file)?;
    let mut rdr = Reader::from_reader(file);
    let headers = rdr.headers()?;

    let aggr = headers
        .iter()
        .position(|s| s == cli.group_by)
        .ok_or(format!(
            "group-by should be one of {:?}, got {}",
            headers, cli.group_by
        ))?;
    let calculated_idx = headers
        .iter()
        .position(|s| s == cli.column_to_calculate)
        .ok_or(format!(
            "file does not have column `{}`",
            cli.column_to_calculate
        ))?;

    let mut map: BTreeMap<String, f64> = BTreeMap::new();

    for record in rdr.records() {
        let record = record?;
        let amount = record
            .get(calculated_idx)
            .ok_or(format!("Record has length 0: {record:?}"))
            .and_then(|v| v.parse::<f64>().map_err(|e| e.to_string()))?;
        let group = record
            .get(aggr)
            .unwrap_or_else(|| panic!("Record should have a field at position {aggr}"));
        let amt = match map.get(group) {
            Some(&amount) => amount,
            None => 0.0,
        };
        map.insert(group.to_string(), amount + amt);
    }

    let mut table = comfy_table::Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Group", "Amount"]);

    let map = match cli.pattern {
        Some(pattern) => {
            let re = RegexSetBuilder::new(pattern)
                .case_insensitive(cli.case_insensitive)
                .build()?;

            map.iter()
                .filter_map(|(k, &v)| {
                    if re.is_match(k) {
                        Some((k.clone(), v))
                    } else {
                        None
                    }
                })
                .collect()
        }
        None => map,
    };

    let mut tuple_vector = map.into_iter().collect::<Vec<_>>();

    if cli.remove_negatives {
        tuple_vector = tuple_vector
            .iter()
            .filter_map(|(k, v)| {
                if *v >= 0.0 {
                    Some((k.clone(), *v))
                } else {
                    None
                }
            })
            .collect();
    } else if cli.remove_positives {
        tuple_vector = tuple_vector
            .iter()
            .filter_map(|(k, v)| {
                if *v < 0.0 {
                    Some((k.clone(), *v))
                } else {
                    None
                }
            })
            .collect();
    }

    tuple_vector.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let total: f64 = tuple_vector.iter().map(|(_, v)| v).sum();

    tuple_vector.iter().for_each(|(k, v)| {
        table.add_row(vec![k, &format!("{v:.2}")]);
    });
    table.add_row(vec![
        Cell::new("TOTAL")
            .add_attribute(Attribute::Bold)
            .fg(Color::Green),
        Cell::new(format!("{total:.2}"))
            .add_attribute(Attribute::Bold)
            .fg(Color::Green),
    ]);

    println!("{table}");
    Ok(())
}
