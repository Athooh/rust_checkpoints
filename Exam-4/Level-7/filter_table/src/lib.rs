#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        if row.len() == self.headers.len() {
            self.body.push(row.to_vec());
        } else {
            panic!("Row length does not match header length");
        }
    }

    pub fn filter_col<F>(&self, filter: F) -> Option<Self>
    where
        F: Fn(&str) -> bool,
    {
        let filtered_indices: Vec<usize> = self
            .headers
            .iter()
            .enumerate()
            .filter_map(|(idx, header)| if filter(header) { Some(idx) } else { None })
            .collect();

        if filtered_indices.is_empty() {
            return None;
        }

        let filtered_headers: Vec<String> = filtered_indices
            .iter()
            .map(|&idx| self.headers[idx].clone())
            .collect();

        let filtered_body: Vec<Vec<String>> = self
            .body
            .iter()
            .map(|row| filtered_indices.iter().map(|&idx| row[idx].clone()).collect())
            .collect();

        Some(Table {
            headers: filtered_headers,
            body: filtered_body,
        })
    }

    pub fn filter_row<F>(&self, col_name: &str, filter: F) -> Option<Self>
    where
        F: Fn(&str) -> bool,
    {
        let col_index = self.headers.iter().position(|header| header == col_name)?;

        let filtered_body: Vec<Vec<String>> = self
            .body
            .iter()
            .filter(|row| filter(&row[col_index]))
            .cloned()
            .collect();

        if filtered_body.is_empty() {
            return None;
        }

        Some(Table {
            headers: self.headers.clone(),
            body: filtered_body,
        })
    }
}