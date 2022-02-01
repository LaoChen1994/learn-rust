pub mod EnumTypesTest {
    #[derive(Debug)]
    pub enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn useMultiTypes() {
        let mut row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Float(3.14),
            SpreadSheetCell::Text(String::from("string")),
        ];

        let rlt = &row.pop();
        for item in &row {
            println!("types -> {:?}", item);
        }

        if let Some(pat) = rlt {
            println!("value => {:?}", pat);
        }
    }
}
