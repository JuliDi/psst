use crate::data::KbShortcut;
use druid::text::{Formatter, Selection, Validation, ValidationError};
use druid::widget::{TextBoxEvent, ValidationDelegate};
use druid::EventCtx;
use std::str::FromStr;

pub struct ShortcutFormatter;

impl Formatter<String> for ShortcutFormatter {
    fn format(&self, value: &String) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &Selection) -> Validation {
        match KbShortcut::from_str(input) {
            Ok(_) => Validation::success(),
            Err(e) => Validation::failure(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Shortcut",
            )),
        }
    }

    fn value(&self, input: &str) -> Result<String, ValidationError> {
        Ok(KbShortcut::from_str(input)
            .or(Err(ValidationError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Shortcut",
            ))))?
            .to_string())
    }
}