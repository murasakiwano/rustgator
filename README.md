# Rustgator - CSV reader and aggregator

This is a very simple CLI that reads a CSV file, aggregate it in groups, and
may perform calculations in it, printing in a nice table.

## Usage

```
read and aggregate CSV files

Usage: rustgator [OPTIONS] --file <FILE> --group-by <GROUP_BY>

Options:
  -f, --file <FILE>          Path to the csv file
  -g, --group-by <GROUP_BY>  Which header to group the records by
  -p, --pattern <PATTERN>    If this flag is present, will group the amounts which match a regex
  -i, --case-insensitive     Whether the matching will be case insensitive (defaults to false)
      --remove-negatives     If negative values should be removes
  -h, --help                 Print help
  -V, --version              Print version
```

Suppose you have a test csv file like that:

```csv
date,category,title,amount
2024-02-10,food,SuperMarkt,20.30
2024-02-11,books,Libre,42.42
2024-02-12,misc,Ferris the Crab,3.14
2024-02-12,food,Sanji,56.56
2024-02-13,clothing,Pappag,29.29
```

Now, if you want to get the total amount by category, you can run something like:

```shell
$ cargo run -- -f test.csv --group-by category
```

It will display a table showing the total amount by category. Using the example
above will result in:

```
╭──────────┬────────╮
│ Group    ┆ Amount │
╞══════════╪════════╡
│ books    ┆ 42.42  │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ clothing ┆ 29.29  │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ food     ┆ 76.86  │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ misc     ┆ 3.14   │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ TOTAL    ┆ 151.71 │
╰──────────┴────────╯
```

`rustgator` assumes the last column in your csv is a floating point value. It
then sums up all the values in that column, grouping by the `--group-by` value.
