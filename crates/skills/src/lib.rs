//! Skills system — fully compatible with Anthropic's Skills format.
//!
//! A "Skill" is a markdown file with YAML front-matter:
//!
//! ```text
//! ---
//! name: bug-hunter
//! description: When the user asks to find or fix a bug, prefer running tests first.
//! ---
//!
//! Steps for bug hunting:
//! 1. Run the test suite to see what fails
//! 2. Read the failing test source
//! ...
//! ```
//!
//! Skills are loaded from `~/.mimo/skills/*.md` and from `./.claude/skills/*.md`
//! (so users can drop their Claude Code skills in unchanged).
//!
//! The agent appends the matched skills' bodies to the system prompt when their
//! description keywords appear in the user's input.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Skills compiled into the binary. Users can override any of them by
/// dropping a same-named `.md` file into `~/.mimo/skills/`.
pub const BUILTIN_SKILLS: &[(&str, &str)] = &[
    ("python-style.md", include_str!("../../../examples/skills/python-style.md")),
    ("rust-style.md", include_str!("../../../examples/skills/rust-style.md")),
    ("git-commits.md", include_str!("../../../examples/skills/git-commits.md")),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub triggers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub frontmatter: SkillFrontmatter,
    pub body: String,
    pub source: PathBuf,
}

impl Skill {
    /// Render the skill as a section that goes into the system prompt.
    pub fn render(&self) -> String {
        format!(
            "<skill name=\"{}\">\n{}\n{}\n</skill>",
            self.frontmatter.name,
            if self.frontmatter.description.is_empty() {
                String::new()
            } else {
                format!("description: {}\n", self.frontmatter.description)
            },
            self.body.trim(),
        )
    }

    /// Check whether this skill should activate for the given user input.
    /// Falls back to keyword match against description if `triggers` is empty.
    pub fn matches(&self, user_input: &str) -> bool {
        let input_l = user_input.to_lowercase();
        if !self.frontmatter.triggers.is_empty() {
            return self
                .frontmatter
                .triggers
                .iter()
                .any(|t| input_l.contains(&t.to_lowercase()));
        }
        // Heuristic: if any 4+ char word in description appears in input, match.
        for word in self.frontmatter.description.split_whitespace() {
            let w = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if w.len() >= 4 && input_l.contains(&w) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Default)]
pub struct SkillRegistry {
    skills: Vec<Skill>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    /// Pre-loaded with the built-in skills shipped inside the binary.
    /// Users override any of them by dropping a same-named file into
    /// `~/.mimo/skills/`.
    pub fn with_builtins() -> Self {
        let mut r = Self::new();
        r.load_builtins();
        r
    }

    /// Loads each built-in skill that's compiled into the binary via
    /// `include_str!`. Failures are logged but never abort startup.
    pub fn load_builtins(&mut self) {
        for (filename, raw) in BUILTIN_SKILLS {
            match Self::parse(filename, raw) {
                Ok(s) => {
                    debug!(skill = %s.frontmatter.name, "loaded built-in");
                    self.skills.push(s);
                }
                Err(e) => warn!(file = %filename, ?e, "built-in skill failed to parse"),
            }
        }
    }

    /// Parse a skill from a raw markdown string + a synthetic source label.
    pub fn parse(source_label: &str, raw: &str) -> Result<Skill> {
        let (fm, body) = parse_frontmatter(raw)
            .ok_or_else(|| anyhow!("no YAML frontmatter in {}", source_label))?;
        let frontmatter: SkillFrontmatter =
            serde_yaml::from_str(fm).with_context(|| format!("yaml in {}", source_label))?;
        Ok(Skill {
            frontmatter,
            body: body.to_string(),
            source: PathBuf::from(format!("<builtin>:{source_label}")),
        })
    }

    /// Load skills from a directory of `.md` files (non-recursive).
    pub fn load_dir(&mut self, dir: &Path) -> Result<usize> {
        if !dir.exists() {
            return Ok(0);
        }
        let mut count = 0;
        for entry in std::fs::read_dir(dir).with_context(|| format!("readdir {}", dir.display()))? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.extension().map(|x| x == "md").unwrap_or(false) {
                match Self::load_file(&path) {
                    Ok(s) => {
                        debug!(skill = %s.frontmatter.name, "loaded");
                        self.skills.push(s);
                        count += 1;
                    }
                    Err(e) => warn!(?path, ?e, "failed to parse skill"),
                }
            }
        }
        Ok(count)
    }

    pub fn load_file(path: &Path) -> Result<Skill> {
        let raw =
            std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let mut s = Self::parse(&path.display().to_string(), &raw)?;
        s.source = path.to_path_buf();
        Ok(s)
    }

    /// Replace any existing skill with the same `name` (so user-supplied skills
    /// override built-ins).
    pub fn dedupe_keep_last(&mut self) {
        let mut seen = std::collections::HashSet::new();
        let mut keep: Vec<Skill> = Vec::with_capacity(self.skills.len());
        for s in self.skills.drain(..).rev() {
            if seen.insert(s.frontmatter.name.clone()) {
                keep.push(s);
            }
        }
        keep.reverse();
        self.skills = keep;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Skill> {
        self.skills.iter()
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    /// Pick the skills that should fire for `user_input`.
    pub fn select_for(&self, user_input: &str) -> Vec<&Skill> {
        self.skills
            .iter()
            .filter(|s| s.matches(user_input))
            .collect()
    }
}

/// Split a markdown file's YAML frontmatter from the body.
/// Returns None if no frontmatter delimiter is present.
fn parse_frontmatter(input: &str) -> Option<(&str, &str)> {
    let rest = input
        .strip_prefix("---\n")
        .or_else(|| input.strip_prefix("---\r\n"))?;
    // Look for closing delimiter.
    let (fm, body) = if let Some(idx) = rest.find("\n---\n") {
        (&rest[..idx], &rest[idx + "\n---\n".len()..])
    } else if let Some(idx) = rest.find("\n---\r\n") {
        (&rest[..idx], &rest[idx + "\n---\r\n".len()..])
    } else if let Some(idx) = rest.find("\n---") {
        (&rest[..idx], rest[idx + "\n---".len()..].trim_start())
    } else {
        return None;
    };
    Some((fm.trim(), body.trim_start()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frontmatter() {
        let raw = "---\nname: foo\ndescription: bar\n---\n\nbody goes here\n";
        let (fm, body) = parse_frontmatter(raw).unwrap();
        assert!(fm.contains("name: foo"));
        assert!(body.contains("body goes here"));
    }

    #[test]
    fn matches_by_description_keyword() {
        let s = Skill {
            frontmatter: SkillFrontmatter {
                name: "bug-hunter".into(),
                description: "When the user asks to find or fix a bug".into(),
                model: None,
                triggers: Vec::new(),
            },
            body: String::new(),
            source: PathBuf::new(),
        };
        assert!(s.matches("can you find this bug"));
        assert!(!s.matches("how do I write a test"));
    }

    #[test]
    fn matches_by_explicit_trigger() {
        let s = Skill {
            frontmatter: SkillFrontmatter {
                name: "x".into(),
                description: String::new(),
                model: None,
                triggers: vec!["deploy".into()],
            },
            body: String::new(),
            source: PathBuf::new(),
        };
        assert!(s.matches("can we deploy this?"));
        assert!(!s.matches("can we test this?"));
    }

    #[test]
    fn renders_into_prompt_section() {
        let s = Skill {
            frontmatter: SkillFrontmatter {
                name: "doc-writer".into(),
                description: "How to write docs".into(),
                model: None,
                triggers: Vec::new(),
            },
            body: "Step 1: outline\nStep 2: write".to_string(),
            source: PathBuf::new(),
        };
        let r = s.render();
        assert!(r.contains("<skill name=\"doc-writer\">"));
        assert!(r.contains("Step 1: outline"));
        assert!(r.ends_with("</skill>"));
    }
}
