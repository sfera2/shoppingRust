#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Product {
    pub name: String,
    pub number: i32,
    pub comment: String,
}

impl Product {
    pub fn new() -> Product {
        return Product {
            name: "".to_string(),
            number: 0,
            comment: "".to_string(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_number(&mut self, number: i32) {
        self.number = number;
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
}