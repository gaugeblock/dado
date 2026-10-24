//! Pattern and mixin registry loader (embedded starter packs + `.dado/` repo overrides).

pub mod verifier;

use crate::error::Result;
use crate::model::{DadoMixin, DadoPattern};
use miter_engine::Language;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

const RUST_PACK: &str = include_str!("../../packs/rust_core.toml");
const TS_PACK: &str = include_str!("../../packs/typescript_core.toml");
const MIXINS_PACK: &str = include_str!("../../packs/core_mixins.toml");

#[derive(Deserialize)]
struct PatternPackFile {
    patterns: Vec<DadoPattern>,
}

#[derive(Deserialize)]
struct MixinPackFile {
    mixins: Vec<DadoMixin>,
}

/// Central registry holding loaded AST patterns and structural mixins.
#[derive(Debug, Clone, Default)]
pub struct PatternRegistry {
    patterns: BTreeMap<String, DadoPattern>,
    mixins: BTreeMap<String, DadoMixin>,
}

impl PatternRegistry {
    /// Creates and initializes a new `PatternRegistry` populated with all embedded packs.
    pub fn new() -> Result<Self> {
        let mut registry = Self::default();
        registry.load_embedded()?;
        Ok(registry)
    }

    /// Loads the embedded 15+ starter patterns and 5 core structural mixins.
    pub fn load_embedded(&mut self) -> Result<()> {
        let rust_pack: PatternPackFile = toml::from_str(RUST_PACK)?;
        for pat in rust_pack.patterns {
            self.patterns.insert(pat.pattern_id.clone(), pat);
        }

        let ts_pack: PatternPackFile = toml::from_str(TS_PACK)?;
        for pat in ts_pack.patterns {
            self.patterns.insert(pat.pattern_id.clone(), pat);
        }

        let mixins_pack: MixinPackFile = toml::from_str(MIXINS_PACK)?;
        for mix in mixins_pack.mixins {
            self.mixins.insert(mix.mixin_id.clone(), mix);
        }

        Ok(())
    }

    fn load_single_dir(&mut self, dir: &Path) -> Result<usize> {
        let mut count = 0;
        let patterns_dir = dir.join("patterns");
        if patterns_dir.exists() && patterns_dir.is_dir() {
            for entry in std::fs::read_dir(patterns_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml") {
                    let content = std::fs::read_to_string(&path)?;
                    if let Ok(pack) = toml::from_str::<PatternPackFile>(&content) {
                        for pat in pack.patterns {
                            self.patterns.insert(pat.pattern_id.clone(), pat);
                            count += 1;
                        }
                    }
                }
            }
        }

        let mixins_dir = dir.join("mixins");
        if mixins_dir.exists() && mixins_dir.is_dir() {
            for entry in std::fs::read_dir(mixins_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml") {
                    let content = std::fs::read_to_string(&path)?;
                    if let Ok(pack) = toml::from_str::<MixinPackFile>(&content) {
                        for mix in pack.mixins {
                            self.mixins.insert(mix.mixin_id.clone(), mix);
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Loads additional patterns or overrides from a directory.
    /// Checks `.gaugeblock/dado`, `.dado`, or `root_dir` directly.
    pub fn load_from_dir(&mut self, root_dir: &Path) -> Result<usize> {
        let mut count = 0;
        let gb_dado = root_dir.join(".gaugeblock").join("dado");
        if gb_dado.is_dir() {
            count += self.load_single_dir(&gb_dado)?;
        }
        let dot_dado = root_dir.join(".dado");
        if dot_dado.is_dir() {
            count += self.load_single_dir(&dot_dado)?;
        }
        if root_dir.join("patterns").is_dir() || root_dir.join("mixins").is_dir() {
            count += self.load_single_dir(root_dir)?;
        }
        Ok(count)
    }

    /// Retrieves a pattern by its identifier.
    pub fn get_pattern(&self, pattern_id: &str) -> Option<&DadoPattern> {
        self.patterns.get(pattern_id)
    }

    /// Retrieves a structural mixin by its identifier.
    pub fn get_mixin(&self, mixin_id: &str) -> Option<&DadoMixin> {
        self.mixins.get(mixin_id)
    }

    /// Lists patterns filtered by optional language and query filter.
    pub fn list_patterns(&self, lang: Option<Language>, query: Option<&str>) -> Vec<&DadoPattern> {
        self.patterns
            .values()
            .filter(|p| {
                if let Some(l) = lang
                    && p.language != l
                    && !(l == Language::TypeScript && p.language == Language::Tsx)
                {
                    return false;
                }
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    return p.pattern_id.to_lowercase().contains(&q_lower)
                        || p.description.to_lowercase().contains(&q_lower);
                }
                true
            })
            .collect()
    }

    /// Lists all registered structural mixins.
    pub fn list_mixins(&self, lang: Option<Language>) -> Vec<&DadoMixin> {
        self.mixins
            .values()
            .filter(|m| {
                if let Some(l) = lang {
                    m.language == l || (l == Language::TypeScript && m.language == Language::Tsx)
                } else {
                    true
                }
            })
            .collect()
    }
}
