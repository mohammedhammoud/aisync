use crate::core::config::SkillMetadata;

pub(crate) struct DefaultSkill {
    pub metadata: SkillMetadata,
    pub content: &'static str,
}

struct EmbeddedDefaultSkill {
    metadata: &'static str,
    content: &'static str,
}

const DEFAULT_SKILLS: &[EmbeddedDefaultSkill] = &[
    EmbeddedDefaultSkill {
        metadata: include_str!("../core/defaults/skills/audit/metadata.json"),
        content: include_str!("../core/defaults/skills/audit/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("../core/defaults/skills/commit/metadata.json"),
        content: include_str!("../core/defaults/skills/commit/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("../core/defaults/skills/debug/metadata.json"),
        content: include_str!("../core/defaults/skills/debug/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("../core/defaults/skills/refactor/metadata.json"),
        content: include_str!("../core/defaults/skills/refactor/SKILL.md"),
    },
];

pub(crate) fn default_instructions() -> &'static str {
    include_str!("../core/defaults/instructions.md")
}

pub(crate) fn default_skills() -> Result<Vec<DefaultSkill>, String> {
    DEFAULT_SKILLS
        .iter()
        .map(|skill| {
            Ok(DefaultSkill {
                metadata: serde_json::from_str(skill.metadata)
                    .map_err(|error| error.to_string())?,
                content: skill.content,
            })
        })
        .collect()
}
