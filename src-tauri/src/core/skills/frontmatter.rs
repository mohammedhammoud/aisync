use crate::core::config::SkillMetadata;
use crate::core::errors::{AppError, AppResult};

use super::types::ParsedSkillContent;

pub fn parse_skill_content(content: &str) -> ParsedSkillContent {
    if !content.starts_with("---\n") {
        return ParsedSkillContent {
            body: content.to_string(),
            frontmatter_lines: Vec::new(),
        };
    }

    let Some(relative_closing_delimiter_index) = content[4..].find("\n---") else {
        return ParsedSkillContent {
            body: content.to_string(),
            frontmatter_lines: Vec::new(),
        };
    };
    let closing_delimiter_index = relative_closing_delimiter_index + 4;

    let mut body_start = closing_delimiter_index + "\n---".len();
    if content[body_start..].starts_with("\r\n") {
        body_start += 2;
    } else if content[body_start..].starts_with('\n') {
        body_start += 1;
    }

    if content[body_start..].starts_with("\r\n") {
        body_start += 2;
    } else if content[body_start..].starts_with('\n') {
        body_start += 1;
    }

    ParsedSkillContent {
        body: content[body_start..].to_string(),
        frontmatter_lines: content[4..closing_delimiter_index]
            .split('\n')
            .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
            .collect(),
    }
}

fn yaml_string(value: &str) -> AppResult<String> {
    serde_json::to_string(value).map_err(AppError::json)
}

pub fn compose_skill_content(
    body: &str,
    metadata: &SkillMetadata,
    frontmatter_lines: &[String],
) -> AppResult<String> {
    let mut frontmatter = vec![
        "---".to_string(),
        format!("name: {}", metadata.id),
        format!("description: {}", yaml_string(&metadata.description)?),
    ];
    frontmatter.extend(
        frontmatter_lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim_start();
                !(trimmed.starts_with("name")
                    && trimmed["name".len()..].trim_start().starts_with(':')
                    || trimmed.starts_with("description")
                        && trimmed["description".len()..].trim_start().starts_with(':'))
            })
            .cloned(),
    );
    frontmatter.push("---".to_string());

    Ok(format!("{}\n\n{}", frontmatter.join("\n"), body))
}

#[cfg(test)]
mod tests {
    use crate::core::config::SkillMetadata;
    use crate::core::skills::{compose_skill_content, parse_skill_content};

    fn metadata(description: &str) -> SkillMetadata {
        SkillMetadata {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            description: description.to_string(),
            enabled: true,
            tags: Vec::new(),
        }
    }

    #[test]
    fn parse_content_without_frontmatter() {
        let parsed = parse_skill_content("# Body\nText");

        assert_eq!(parsed.body, "# Body\nText");
        assert!(parsed.frontmatter_lines.is_empty());
    }

    #[test]
    fn parse_content_with_frontmatter() {
        let parsed = parse_skill_content("---\nname: audit\ndescription: Test\n---\n# Body");

        assert_eq!(parsed.body, "# Body");
        assert_eq!(
            parsed.frontmatter_lines,
            vec!["name: audit", "description: Test"]
        );
    }

    #[test]
    fn parse_content_with_extra_blank_line_after_delimiter() {
        let parsed = parse_skill_content("---\nname: audit\n---\n\n# Body");

        assert_eq!(parsed.body, "# Body");
        assert_eq!(parsed.frontmatter_lines, vec!["name: audit"]);
    }

    #[test]
    fn compose_filters_name_and_description_frontmatter() {
        let content = compose_skill_content(
            "# Body",
            &metadata("New description"),
            &[
                "name: old".to_string(),
                "description: old".to_string(),
                "tags: [safe]".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(
            content,
            "---\nname: test-skill\ndescription: \"New description\"\ntags: [safe]\n---\n\n# Body"
        );
    }

    #[test]
    fn compose_escapes_description() {
        let content = compose_skill_content("Body", &metadata("A \"quoted\" value"), &[]).unwrap();

        assert!(content.contains("description: \"A \\\"quoted\\\" value\""));
    }
}
