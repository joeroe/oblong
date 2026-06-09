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

    pub fn push_empty_row(&mut self) {
        let row = self.columns
            .iter()
            .map(|_| CellValue::Empty)
            .collect();
        self.rows.push(row);
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
}
