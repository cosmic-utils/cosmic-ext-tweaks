use std::borrow::Cow;

use ron::{
    de::Deserializer,
    error::{Error, Position, SpannedError, SpannedResult},
};
use serde::{Deserialize, de::DeserializeOwned};

use super::config::padding_override;

pub fn from_str<T: DeserializeOwned>(input: &str) -> SpannedResult<T> {
    let mut input = Cow::Borrowed(input);
    loop {
        match ron::from_str(&input) {
            Ok(value) => return Ok(value),
            Err(error) => convert_legacy_field(&mut input, &error).ok_or(error)?,
        }
    }
}

fn convert_legacy_field(input: &mut Cow<'_, str>, error: &SpannedError) -> Option<()> {
    let Error::NoSuchStructField {
        found,
        outer: Some(outer),
        ..
    } = &error.code
    else {
        return None;
    };
    if outer != "CosmicPanelConfig" {
        return None;
    }

    // RON identifies the actual field, including when the same text appears in a string.
    let start = byte_offset(input, &error.span.start)?;
    let after_key = byte_offset(input, &error.span.end)?;
    let suffix = Deserializer::from_str(input.get(after_key..)?).ok()?;
    let mut value = Deserializer::from_str(suffix.remainder().strip_prefix(':')?).ok()?;
    let number = u32::deserialize(&mut value).ok()?;
    let (key, override_value) = match found.as_str() {
        "padding" => ("padding_override", padding_override(number)),
        "border_radius" => ("border_radius_override", Some(number)),
        _ => return None,
    };
    let end = input.len() - value.remainder().len();
    input
        .to_mut()
        .replace_range(start..end, &format!("{key}: {override_value:?}"));
    Some(())
}

fn byte_offset(input: &str, position: &Position) -> Option<usize> {
    let line_starts = std::iter::once(0).chain(input.match_indices('\n').map(|(i, _)| i + 1));
    let start = line_starts.into_iter().nth(position.line - 1)?;
    let line = input[start..].split('\n').next()?;
    let offsets = line.char_indices().map(|(i, _)| i);
    let column = offsets
        .chain(std::iter::once(line.len()))
        .nth(position.col - 1)?;
    Some(start + column)
}
