use chrono_tz::TZ_VARIANTS;
use inquire::{Autocomplete, autocompletion::Replacement};

#[derive(Clone)]
pub struct TimezoneAutocomplete;

impl Autocomplete for TimezoneAutocomplete {
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, inquire::CustomUserError> {
        if let Some(suggestion) = highlighted_suggestion {
            return Ok(Replacement::Some(suggestion));
        }

        // If there's only one clear suggestion, complete to it
        let suggestions = self.get_suggestions(input)?;
        if suggestions.len() == 1 {
            return Ok(Replacement::Some(suggestions[0].clone()));
        }

        Ok(Replacement::None)
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let variants = TZ_VARIANTS
            .iter()
            .filter(|tz| {
                tz.to_string()
                    .to_lowercase()
                    .contains(&input.to_lowercase())
            })
            .map(|tz| tz.to_string())
            .collect::<Vec<_>>();
        Ok(variants)
    }
}
