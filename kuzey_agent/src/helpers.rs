use inquire::{
    CustomUserError,
    autocompletion::{Autocomplete, Replacement},
};
use std::{fs, path::Path};

/// Suggests file paths when the current word starts with '@'.
#[derive(Clone, Default)]
pub struct MentionCompleter;

/// Returns (text before the '@', partial path after it) if the cursor is in a mention.
fn split_mention(input: &str) -> Option<(&str, &str)> {
    let start = input.rfind('@')?;
    let before = &input[..start];
    let partial = &input[start + 1..];

    // Only count '@' at the start of a word (so "me@mail.com" is ignored),
    // and stop once the user typed a space after the path.
    if !(before.is_empty() || before.ends_with(char::is_whitespace))
        || partial.contains(char::is_whitespace)
    {
        return None;
    }
    Some((before, partial))
}

impl Autocomplete for MentionCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let Some((_, partial)) = split_mention(input) else {
            return Ok(vec![]);
        };

        // "src/ma" -> look in "src/" for names starting with "ma".
        let (dir, prefix) = match partial.rfind('/') {
            Some(i) => (&partial[..=i], &partial[i + 1..]),
            None => ("", partial),
        };
        let Ok(entries) = fs::read_dir(if dir.is_empty() { "." } else { dir }) else {
            return Ok(vec![]);
        };

        let mut suggestions: Vec<String> = entries
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                // Hide dotfiles unless the user started typing a '.'.
                if !name.starts_with(prefix) || (name.starts_with('.') && !prefix.starts_with('.')) {
                    return None;
                }
                let slash = if entry.path().is_dir() { "/" } else { "" };
                Some(format!("{dir}{name}{slash}"))
            })
            .collect();
        suggestions.sort();
        suggestions.truncate(15);
        Ok(suggestions)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        let Some((before, _)) = split_mention(input) else {
            return Ok(None);
        };

        // Use the suggestion picked with the arrow keys, or the only match if there is one.
        let pick = match highlighted {
            Some(s) => s,
            None => match self.get_suggestions(input)?.as_slice() {
                [only] => only.clone(),
                _ => return Ok(None),
            },
        };
        Ok(Some(format!("{before}@{pick}")))
    }
}

/// Appends the contents of every `@file` in the message so the model can see them.
pub fn expand_mentions(input: &str) -> String {
    let mut out = input.to_string();
    for word in input.split_whitespace() {
        let Some(path) = word.strip_prefix('@') else {
            continue;
        };
        if !Path::new(path).is_file() {
            continue;
        }
        match fs::read_to_string(path) {
            Ok(content) => out.push_str(&format!("\n\n<file path=\"{path}\">\n{content}\n</file>")),
            Err(e) => eprintln!("Could not read {path}: {e}"),
        }
    }
    out
}
