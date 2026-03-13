/*
Prompt and context sanitization layer.

Responsibilities:
    - minimize data before model invocation
    - mask or redact sensitive content
    - prepare model-safe text representation
*/
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
// ToDO
// regex PII
// field masking
// full Microsoft Presidio
