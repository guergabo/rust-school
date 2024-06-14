use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = CSVParser::new();
    let mut processor = CSVProcessor::new(&mut parser);
    processor.process_csv()?;

    Ok(())
}

struct CSVProcessor<'a> {
    parser: &'a mut CSVParser,
}

impl<'a> CSVProcessor<'a> {
    fn new(parser: &'a mut CSVParser) -> Self {
        CSVProcessor { parser }
    }

    fn process_csv(&mut self) -> Result<(), Box<dyn Error>> {
        self.parser.parse_csv("ownership/data.csv")?;

        if let Some(row) = self.parser.get_row(1) {
            println!("Row 1: {:?}", row);
        }
        if let Some(cell) = self.parser.get_cell(2, 3) {
            println!("Cell (2, 3): {}", cell);
        }

        self.parser.update_cell(2, 3, "Updated Value")?;
        self.parser.write_csv("ownership/updated_data.csv")?;
        self.parser.display_csv()?;

        Ok(())
    }
}

type Row = Vec<String>;

struct CSVParser {
    data: Vec<Row>,
}

impl CSVParser {
    fn new() -> Self {
        CSVParser { data: Vec::new() }
    }

    fn parse_csv(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let values: Row = line.split(",").map(|s| s.to_string()).collect();
            self.data.push(values);
        }
        Ok(())
    }

    fn get_row(&self, row_index: usize) -> Option<&Row> {
        self.data.get(row_index)
    }

    fn get_cell(&self, row_index: usize, col_index: usize) -> Option<&str> {
        self.data
            .get(row_index)
            .and_then(|row| row.get(col_index))
            .map(|cell| cell.as_str())
    }

    fn update_cell(
        &mut self,
        row_index: usize,
        col_index: usize,
        value: &str,
    ) -> Result<(), String> {
        match self.data.get_mut(row_index) {
            Some(row) => match row.get_mut(col_index) {
                Some(cell) => {
                    *cell = value.to_string();
                    Ok(())
                }
                None => Err(format!("Invalid column index: {}", col_index)),
            },
            None => Err(format!("Invalid row index: {}", row_index)),
        }
    }

    fn write_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_path)?;
        for row in &self.data {
            let line = row.join(",");
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    fn display_csv(&self) -> Result<(), Box<dyn Error>> {
        for row in &self.data {
            let line = row.join(",");
            println!("{}", line);
        }
        Ok(())
    }
}
