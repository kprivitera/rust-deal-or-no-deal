use prettytable::{Table, Row, Cell};

pub struct GameTable {
  pub table: Table,
}

impl GameTable {
  pub fn new(sorted_cases: Vec<(&u32, &f64)>, selected_cases: &Vec<u32>) -> GameTable {
    let (column1, column2) = GameTable::populate_columns(sorted_cases, selected_cases);
    Self::create_table(column1, column2)
  }

  pub fn get_style_spec(i: usize, status: &str) -> &'static str {
    if !status.is_empty() {
        "Fw"
    } else if i < 11 {
        "Fb"
    } else if i < 17 {
        "Fr"
    } else {
        "Fg"
    }
  }

  pub fn print(&self) {
    self.table.printstd();
  }

  fn populate_columns(sorted_cases: Vec<(&u32, &f64)>, selected_cases: &Vec<u32>) -> (Vec<Cell>, Vec<Cell>) {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();
    for (i, (&case_number, &case_value)) in sorted_cases.iter().enumerate() {
        let status = if selected_cases.contains(&case_number) { " (opened)" } else { "" };
        let output = format!("{}{}", case_value, status);
        let style_spec = GameTable::get_style_spec(i, status);
        let cell = Cell::new(&output).style_spec(style_spec);

        if i < 11 {
            column1.push(cell);
        } else {
            column2.push(cell);
        }
    }
    (column1, column2)
  }

  fn create_table(column1: Vec<Cell>, column2: Vec<Cell>) -> GameTable {
    let mut table = Table::new();
    for i in 0..column1.len() {
        let mut row = vec![column1[i].clone()];
        if i < column2.len() {
            row.push(column2[i].clone());
        }
        table.add_row(Row::new(row));
    }
    GameTable { table }
  }
}