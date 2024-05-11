/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use log::error;

use crate::{
    error::{ExecError, ExecResult},
    templating_engine::Preprocessor,
};

#[derive(Debug, Clone)]
struct Template {
    pub id: String,
    pub literal: String,
}

impl Template {
    pub fn load<'a, P: AsRef<Path>>(path: P) -> ExecResult<Template> {
        let literal = Self::read_template_str(&path.as_ref())?;
        let preproc = Preprocessor::run(&literal)?;

        Ok(Self {
            id: preproc.id.to_string(),
            literal,
        })
    }

    fn read_template_str<P: AsRef<Path>>(path: P) -> ExecResult<String> {
        Ok(fs::read_to_string(&path).map_err(|e| ExecError::FileReadWriteError(e.to_string()))?)
    }
}

// templates are stored in hashsets via 'template set entries'
// this way we can index templates by a value inside their structs, specifically their ID
#[derive(Debug, Clone)]
struct TemplateSetEntry(Template);

impl Eq for TemplateSetEntry {}

// compare template structs by their ID
impl PartialEq<TemplateSetEntry> for TemplateSetEntry {
    fn eq(&self, other: &TemplateSetEntry) -> bool {
        self.0.id == other.0.id
    }
}

// index template structs by hashing their ID
impl std::hash::Hash for TemplateSetEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
    }
}

// this allows us to look up a template in the set ysing &str refs
impl std::borrow::Borrow<str> for TemplateSetEntry {
    fn borrow(&self) -> &str {
        &self.0.id
    }
}

#[derive(Debug, Clone, Default)]
pub struct TemplateSet {
    file_templates: HashSet<TemplateSetEntry>,
    project_templates: HashSet<TemplateSetEntry>,
}

impl TemplateSet {
    pub fn new() -> Self {
        Self {
            file_templates: HashSet::new(),
            project_templates: HashSet::new(),
        }
    }

    pub fn load_file_templates<P: AsRef<Path>>(mut self, dir: P) -> ExecResult<Self> {
        let paths = Self::read_templates_dir(dir)?;
        Self::load_templates_from_path_list(&mut self.file_templates, paths)?;

        Ok(self)
    }

    pub fn load_project_templates<P: AsRef<Path>>(mut self, dir: P) -> ExecResult<Self> {
        let paths = Self::read_templates_dir(dir)?;
        Self::load_templates_from_path_list(&mut self.project_templates, paths)?;

        Ok(self)
    }

    fn read_templates_dir<P: AsRef<Path>>(path: P) -> ExecResult<Vec<PathBuf>> {
        let mut buf = vec![];
        let Ok(entries) = fs::read_dir(path) else {
            return Ok(buf);
        };

        for entry in entries {
            // get entry (file/folder) metadata
            let entry = entry.map_err(|e| ExecError::FileReadWriteError(e.to_string()))?;
            let meta = entry
                .metadata()
                .map_err(|e| ExecError::FileReadWriteError(e.to_string()))?;

            if meta.is_dir() {
                let mut subdir = Self::read_templates_dir(entry.path())?;
                buf.append(&mut subdir);
            }
            if meta.is_file() {
                buf.push(entry.path());
            }
        }

        Ok(buf)
    }

    fn load_templates_from_path_list<P: AsRef<Path>>(
        set: &mut HashSet<TemplateSetEntry>,
        paths: Vec<P>,
    ) -> ExecResult<()> {
        Ok(for p in paths.iter() {
            let t = TemplateSetEntry(Template::load(p)?);

            // check against collisions, warn the user if there are multiple templates with the same name/id
            if set.contains(&t) {
                error!("Found duplicated template id: \"{}\"", t.0.id);
                continue;
            }
            set.insert(t);
        })
    }
}
