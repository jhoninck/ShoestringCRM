pub struct Redactor {}

impl Redactor {

    pub fn new() -> Self {
        Self {}
    }

    pub fn clean(&self, input: &str) -> String {

        let mut text = input.to_string();

        text = text.replace("@", "[at]");

        text
    }
}
