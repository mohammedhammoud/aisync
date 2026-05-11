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
