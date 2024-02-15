# Rustgator - CSV reader and aggregator

This is a very simple CLI that reads a CSV file, aggregate it in groups, and
may perform calculations in it, printing in a nice table.

## Usage

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

It will display a table showing the total amount by category.
