use super::value::DataValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dataset {
    pub headers: Vec<String>,
    pub records: Vec<HashMap<String, DataValue>>,
}

impl Dataset {
    pub fn new(headers: Vec<String>, records: Vec<HashMap<String, DataValue>>) -> Self {
        Dataset { headers, records }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.records.len(), self.headers.len())
    }
}
