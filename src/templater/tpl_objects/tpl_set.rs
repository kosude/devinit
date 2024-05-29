/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{
    collections::HashSet,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use log::error;

use crate::{
    error::{DevinitError, DevinitResult},
    templater::{Context, ContextArcMutex},
};

use super::{FileTemplate, ProjectTemplate, Template};

/// Templates are stored in hashsets via 'template set entries'.
/// This way we can index templates by a value inside their structs, specifically their ID
#[derive(Debug, Clone)]
struct TemplateSetEntry<'a, T: Template<'a>>(T, PhantomData<&'a T>);

impl<'a, T: Template<'a>> Eq for TemplateSetEntry<'a, T> {}
impl<'a, T: Template<'a>> PartialEq<TemplateSetEntry<'a, T>> for TemplateSetEntry<'a, T> {
    fn eq(&self, other: &TemplateSetEntry<'a, T>) -> bool {
        self.0.name() == other.0.name()
    }
}
impl<'a, T: Template<'a>> std::hash::Hash for TemplateSetEntry<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
    }
}
impl<'a, T: Template<'a>> std::borrow::Borrow<str> for TemplateSetEntry<'a, T> {
    fn borrow(&self) -> &str {
        &self.0.name()
    }
}

/// A structure containing a set of each type of template available to the user.
#[derive(Debug, Clone, Default)]
pub struct TemplateSet<'a> {
    ctx: ContextArcMutex,

    file_templates: HashSet<TemplateSetEntry<'a, FileTemplate>>,
    project_templates: HashSet<TemplateSetEntry<'a, ProjectTemplate>>,
}

impl<'a> TemplateSet<'a> {
    pub fn new() -> Self {
        Self {
            ctx: Arc::new(Mutex::new(Context::new())),
            file_templates: HashSet::new(),
            project_templates: HashSet::new(),
        }
    }

    pub fn load_file_templates<P: AsRef<Path>>(mut self, dir: P) -> DevinitResult<Self> {
        let paths = Self::read_templates_dir(dir, false)?;
        let ctx = &self.ctx;
        Self::load_templates_from_path_list(&mut self.file_templates, paths, |p| {
            FileTemplate::load(p, ctx.clone())
        })?;

        Ok(self)
    }

    pub fn load_project_templates<P: AsRef<Path>>(mut self, dir: P) -> DevinitResult<Self> {
        let paths = Self::read_templates_dir(dir, true)?;
        let ctx = &self.ctx;
        Self::load_templates_from_path_list(&mut self.project_templates, paths, |p| {
            ProjectTemplate::load(p, ctx.clone())
        })?;

        Ok(self)
    }

    /// Recursively read through a directory for template config scripts
    fn read_templates_dir<P: AsRef<Path>>(path: P, projects: bool) -> DevinitResult<Vec<PathBuf>> {
        let mut buf = vec![];
        let Ok(entries) = fs::read_dir(path) else {
            return Ok(buf);
        };

        for entry in entries {
            // get entry (file/folder) metadata
            let entry = entry.map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;
            let meta = entry
                .metadata()
                .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;

            if meta.is_dir() {
                let mut subdir = Self::read_templates_dir(entry.path(), projects)?;
                buf.append(&mut subdir);
            }
            if meta.is_file() {
                if !projects || (entry.file_name() == "templaterc.yml") {
                    buf.push(entry.path());
                }
            }
        }

        Ok(buf)
    }

    /// Load each template in the list of file paths given in `paths`, each one referring to a template configuration script.
    /// The function to load a template is given via the `load_func` parameter.
    fn load_templates_from_path_list<
        P: AsRef<Path>,
        T: Template<'a>,
        F: Fn(&P) -> DevinitResult<T>,
    >(
        set: &mut HashSet<TemplateSetEntry<'a, T>>,
        paths: Vec<P>,
        load_func: F,
    ) -> DevinitResult<()> {
        Ok(for p in paths.iter() {
            let t = TemplateSetEntry(load_func(p)?, PhantomData);

            // check against collisions, warn the user if there are multiple templates with the same name/id
            if set.contains(&t) {
                error!("Found duplicate template id: \"{}\"\n", t.0.name());
                continue;
            }
            set.insert(t);
        })
    }

    /// Retrieve a file template from the set
    pub fn get_file_template(&self, id: &str) -> DevinitResult<&FileTemplate> {
        Ok(&self
            .file_templates
            .get(id)
            .ok_or(DevinitError::IdNotFoundError(format!("\"{}\" (FILE)", id)))?
            .0)
    }

    /// Retrieve a project template from the set
    pub fn get_project_template(&self, id: &str) -> DevinitResult<&ProjectTemplate> {
        Ok(&self
            .project_templates
            .get(id)
            .ok_or(DevinitError::IdNotFoundError(format!(
                "\"{}\" (PROJECT)",
                id
            )))?
            .0)
    }

    /// Get a list of references to all loaded file templates
    pub fn get_file_templates_all(&self) -> Vec<&FileTemplate> {
        self.file_templates.iter().map(|e| &e.0).collect()
    }

    /// Get a list of references to all loaded project templates
    pub fn get_project_templates_all(&self) -> Vec<&ProjectTemplate> {
        self.project_templates.iter().map(|e| &e.0).collect()
    }
}
