/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use crate::{
    error::{ExecError, ExecResult},
    templater::Preprocessor,
};

use log::error;

use std::{
    collections::HashSet,
    fmt, fs,
    path::{Path, PathBuf},
};

/// A generic template trait that encompasses (and is implemented by) both file and project templates.
pub trait Template: fmt::Display + fmt::Debug + Clone {
    fn load<P: AsRef<Path>>(path: P) -> ExecResult<impl Template>;

    fn id(&self) -> &String;
    fn literal(&self) -> &String;
}

/// A template to initialise a single file
#[derive(Debug, Clone)]
pub struct FileTemplate {
    id: String,
    literal: String,
}

impl Template for FileTemplate {
    /// Load the file template from a single template configuration script
    fn load<P: AsRef<Path>>(path: P) -> ExecResult<FileTemplate> {
        let literal =
            fs::read_to_string(&path).map_err(|e| ExecError::FileReadWriteError(e.to_string()))?;
        let preproc = Preprocessor::run(&literal)?;

        Ok(Self {
            id: preproc.id.to_string(),
            literal: preproc.clean_literal,
        })
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn literal(&self) -> &String {
        &self.literal
    }
}

/// A template to initialise a project (i.e. a directory of files)
#[derive(Debug, Clone)]
pub struct ProjectTemplate {}

impl Template for ProjectTemplate {
    /// Load the project template from a configuration file in addition to any template configuration scripts
    fn load<P: AsRef<Path>>(_path: P) -> ExecResult<ProjectTemplate> {
        todo!()
    }

    fn id(&self) -> &String {
        todo!()
    }

    fn literal(&self) -> &String {
        todo!()
    }
}

/// Templates are stored in hashsets via 'template set entries'.
/// This way we can index templates by a value inside their structs, specifically their ID
#[derive(Debug, Clone)]
struct TemplateSetEntry<T: Template>(T);

impl<T: Template> Eq for TemplateSetEntry<T> {}
impl<T: Template> PartialEq<TemplateSetEntry<T>> for TemplateSetEntry<T> {
    fn eq(&self, other: &TemplateSetEntry<T>) -> bool {
        self.0.id() == other.0.id()
    }
}
impl<T: Template> std::hash::Hash for TemplateSetEntry<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id().hash(state);
    }
}
impl<T: Template> std::borrow::Borrow<str> for TemplateSetEntry<T> {
    fn borrow(&self) -> &str {
        &self.0.id()
    }
}

/// A structure containing a set of each type of template available to the user.
#[derive(Debug, Clone, Default)]
pub struct TemplateSet {
    file_templates: HashSet<TemplateSetEntry<FileTemplate>>,
    project_templates: HashSet<TemplateSetEntry<ProjectTemplate>>,
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
        Self::load_templates_from_path_list(&mut self.file_templates, paths, |p| {
            FileTemplate::load(p)
        })?;

        Ok(self)
    }

    pub fn load_project_templates<P: AsRef<Path>>(mut self, dir: P) -> ExecResult<Self> {
        let paths = Self::read_templates_dir(dir)?;
        Self::load_templates_from_path_list(&mut self.project_templates, paths, |p| {
            ProjectTemplate::load(p)
        })?;

        Ok(self)
    }

    /// Recursively read through a directory for template config scripts
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

    /// Load each template in the list of file paths given in `paths`, each one referring to a template configuration script.
    /// The function to load a template is given via the `load_func` parameter.
    fn load_templates_from_path_list<P: AsRef<Path>, T: Template, F: Fn(&P) -> ExecResult<T>>(
        set: &mut HashSet<TemplateSetEntry<T>>,
        paths: Vec<P>,
        load_func: F,
    ) -> ExecResult<()> {
        Ok(for p in paths.iter() {
            let t = TemplateSetEntry(load_func(p)?);

            // check against collisions, warn the user if there are multiple templates with the same name/id
            if set.contains(&t) {
                error!("Found duplicated template id: \"{}\"", t.0.id());
                continue;
            }
            set.insert(t);
        })
    }

    /// Retrieve a file template from the set
    pub fn get_file_template(&self, id: &str) -> ExecResult<&FileTemplate> {
        Ok(&self
            .file_templates
            .get(id)
            .ok_or(ExecError::IdNotFoundError(format!("\"{}\" (FILE)", id)))?
            .0)
    }

    /// Retrieve a project template from the set
    pub fn get_project_template(&self, id: &str) -> ExecResult<&ProjectTemplate> {
        Ok(&self
            .project_templates
            .get(id)
            .ok_or(ExecError::IdNotFoundError(format!("\"{}\" (PROJECT)", id)))?
            .0)
    }
}
