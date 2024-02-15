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
    #[arg(short, long)]
    file: PathBuf,

    /// Which header to group the records by
    #[arg(short, long)]
    group_by: String,

    /// If this flag is present, will group the amounts which match a regex
    #[arg(short, long)]
    pattern: Option<Vec<String>>,

    /// Whether the matching will be case insensitive (defaults to false)
    #[arg(short = 'i', long)]
    case_insensitive: bool,

    /// If negative values should be removes
    #[arg(long)]
    remove_negatives: bool,
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

    let mut map: BTreeMap<String, f64> = BTreeMap::new();

    for record in rdr.records() {
        let record = record?;
        let amount = record
            .get(record.len() - 1)
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

    let mut map = match cli.pattern {
        Some(pattern) => {
            let re = RegexSetBuilder::new(pattern)
                .case_insensitive(cli.case_insensitive)
                .build()?;

            map.iter()
                .filter_map(|(k, &v)| {
                    if re.is_match(k) {
                        table.add_row(Vec::from(&[k, &format!("{v:.2}")]));
                        Some((k.clone(), v))
                    } else {
                        None
                    }
                })
                .collect()
        }
        None => {
            map.iter().for_each(|(k, v)| {
                table.add_row(Vec::from(&[k, &format!("{v:.2}")]));
            });

            map
        }
    };

    if cli.remove_negatives {
        map = map
            .iter()
            .filter_map(|(k, &v)| if v >= 0.0 { Some((k.clone(), v)) } else { None })
            .collect();
    }

    let total: f64 = map.values().sum();

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
