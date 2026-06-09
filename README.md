# oblong

**oblong** is a terminal-based tabular data editor written in Rust. It focuses
on producing typed, rectangular, column-oriented tables ready to import into
data science workflows in R, Python, Julia, etc. It is 
[not a spreadsheet](#oblong-is-not-a-spreadsheet).

It features:

- Strictly rectangular, column-oriented tables
- Explicit column types (currently integer, float, text, or boolean)
- Modal, vim-like keyboard interface
- Fast, memory-safe performance even with large tables

## oblong is not a spreadsheet

oblong is *not* a spreadsheet program like LibreOffice Calc, Microsoft Excel,
or Google Sheets. This type of software was originally designed for accountancy 
and, though widely used, they encourage a number of anti-patterns when used to
[prepare data for statistical analysis](https://www.tandfonline.com/doi/full/10.1080/00031305.2017.1375989).

This means that oblong purposely does not support: 

- Non-rectangular tables
- Row-oriented tables (variables as rows; observations as columns)
- Typeless or implicitly typed columns
- In-cell calculations or 'formulas'
- Cross-references between cells or tables

If you are looking for a terminal-based spreadsheet, try 
[sc-im](https://github.com/andmarti1424/sc-im).
