// src/model.rs
// Table data model

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    Float,
    Text,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

pub type Row = Vec<CellValue>;

#[derive(Debug)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        assert!(!columns.is_empty(), "table must have at least one column");
        Table {
            columns,
            rows: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn insert_empty_row(&mut self, index: usize) {
        let row = self.columns
            .iter()
            .map(|_| CellValue::Empty)
            .collect();

        let idx = index.min(self.rows.len());
        self.rows.insert(idx, row);
    }

    pub fn push_empty_row(&mut self) {
        self.insert_empty_row(self.rows.len());
    }

    pub fn insert_column(&mut self, index: usize, column: Column) {
        let idx = index.min(self.columns.len());

        // Insert the column metadata
        self.columns.insert(idx, column);

        // Insert an empty cell into every existing row
        for row in &mut self.rows {
            row.insert(idx, CellValue::Empty);
        }
    }

}

impl ColumnType {
    pub fn parse(&self, input: &str) -> Result<CellValue, String> {
        let input = input.trim();

        if input.is_empty() {
            return Ok(CellValue::Empty);
        }

        match self {
            ColumnType::Integer => input
                .parse::<i64>()
                .map(CellValue::Int)
                .map_err(|_| "expected integer".to_string()),

            ColumnType::Float => input
                .parse::<f64>()
                .map(CellValue::Float)
                .map_err(|_| "expected float".to_string()),

            ColumnType::Text => Ok(CellValue::Text(input.to_string())),

            ColumnType::Boolean => match input {
                "true" | "TRUE" | "1" => Ok(CellValue::Bool(true)),
                "false" | "FALSE" | "0" => Ok(CellValue::Bool(false)),
                _ => Err("expected boolean (true/false)".to_string()),
            },
        }
    }
}

impl CellValue {
    pub fn to_edit_string(&self) -> String {
        match self {
            CellValue::Int(v) => v.to_string(),
            CellValue::Float(v) => v.to_string(),
            CellValue::Text(v) => v.clone(),
            CellValue::Bool(v) => v.to_string(),
            CellValue::Empty => String::new(),
        }
    }
}

pub fn commit_edit(
    table: &mut Table,
    row: usize,
    col: usize,
    input: &str,
) -> Result<(), String> {
    let column = &table.columns[col];
    let value = column.col_type.parse(input)?;
    table.rows[row][col] = value;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_valid() {
        let ct = ColumnType::Integer;
        assert_eq!(ct.parse("42").unwrap(), CellValue::Int(42));
    }

    #[test]
    fn parse_integer_invalid() {
        let ct = ColumnType::Integer;
        assert!(ct.parse("abc").is_err());
    }

    #[test]
    fn parse_empty_is_empty_cell() {
        let ct = ColumnType::Text;
        assert_eq!(ct.parse("").unwrap(), CellValue::Empty);
    }

    #[test]
    fn table_is_rectangular() {
        let columns = vec![
            Column { name: "id".into(), col_type: ColumnType::Integer },
            Column { name: "value".into(), col_type: ColumnType::Float },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();
        table.push_empty_row();

        for row in &table.rows {
            assert_eq!(row.len(), table.width());
        }
    }

    #[test]
    #[should_panic]
    fn table_requires_at_least_one_column() {
        let _ = Table::new(vec![]);
    }

    #[test]
    fn commit_valid_integer_edit() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();

        let result = commit_edit(&mut table, 0, 0, "42");

        assert!(result.is_ok());
        assert_eq!(table.rows[0][0], CellValue::Int(42));
    }

    #[test]
    fn commit_invalid_integer_edit_fails_and_does_not_modify_cell() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();

        let result = commit_edit(&mut table, 0, 0, "abc");

        assert!(result.is_err());
        assert_eq!(table.rows[0][0], CellValue::Empty);
    }

    #[test]
    fn commit_empty_input_results_in_empty_cell() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();

        let result = commit_edit(&mut table, 0, 0, "");

        assert!(result.is_ok());
        assert_eq!(table.rows[0][0], CellValue::Empty);
    }

    #[test]
    fn insert_empty_row_into_empty_table() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);

        table.insert_empty_row(0);

        assert_eq!(table.height(), 1);
        assert_eq!(table.rows[0].len(), table.width());
        assert!(matches!(table.rows[0][0], CellValue::Empty));
    }

    #[test]
    fn insert_empty_row_above_existing_row() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();

        table.insert_empty_row(0);

        assert_eq!(table.height(), 2);
        assert!(matches!(table.rows[0][0], CellValue::Empty));
        assert!(matches!(table.rows[1][0], CellValue::Empty));
    }

    #[test]
    fn insert_empty_row_below_existing_row() {
        let columns = vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ];

        let mut table = Table::new(columns);
        table.push_empty_row();

        table.insert_empty_row(1);

        assert_eq!(table.height(), 2);
    }

    #[test]
    fn insert_column_into_empty_table() {
        let mut table = Table::new(vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ]);

        table.insert_column(
            1,
            Column {
                name: "value".into(),
                col_type: ColumnType::Float,
            },
        );

        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn insert_column_adds_empty_cells_to_all_rows() {
        let mut table = Table::new(vec![
            Column {
                name: "id".into(),
                col_type: ColumnType::Integer,
            },
        ]);

        table.push_empty_row();
        table.push_empty_row();

        table.insert_column(
            1,
            Column {
                name: "value".into(),
                col_type: ColumnType::Float,
            },
        );

        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.rows.len(), 2);

        for row in &table.rows {
            assert_eq!(row.len(), 2);
            assert!(matches!(row[1], CellValue::Empty));
        }
    }

}
