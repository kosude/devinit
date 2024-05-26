/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use super::{Context, ContextArcMutex, FileRenderer, ProjectRenderer, Renderer, RendererVariant};
use crate::{
    error::{DevinitError, DevinitResult},
    files::ProjectTemplateYamlBuilder,
};
use log::error;
use miette::IntoDiagnostic;
use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// A generic template trait that encompasses (and is implemented by) both file and project templates.
pub trait Template<'a>: fmt::Debug + Clone {
    type Me;

    fn load<P: AsRef<Path>>(path: P, ctx: ContextArcMutex) -> DevinitResult<Self::Me>;

    fn name(&self) -> &String;
    fn source(&self) -> &String;

    fn context(&self) -> ContextArcMutex;
    fn make_renderer(&'a self) -> DevinitResult<RendererVariant>;
}

/// A template to initialise a single file
#[derive(Debug, Clone)]
pub struct FileTemplate {
    ctx_ref: ContextArcMutex,

    name: String,
    source: String,
}

impl<'a> Template<'a> for FileTemplate {
    type Me = Self;

    /// Load the file template from a single template configuration script
    fn load<P: AsRef<Path>>(path: P, ctx: ContextArcMutex) -> DevinitResult<Self::Me> {
        let name = String::from(
            path.as_ref()
                .file_name()
                .ok_or(DevinitError::FileReadWriteError(format!(
                    "Failed to extract filename from path {:?}",
                    path.as_ref()
                )))?
                .to_str()
                .unwrap(),
        );
        let literal = fs::read_to_string(&path)
            .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;

        let mut ctx_lock = ctx.lock().unwrap();
        ctx_lock
            .tera_mut()
            .add_raw_template(&name, &literal)
            .into_diagnostic()
            .map_err(|e| DevinitError::TemplateParseError(format!("{:?}", e)))?;

        Ok(Self {
            ctx_ref: ctx.clone(),
            name,
            source: path.as_ref().display().to_string(),
        })
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn context(&self) -> ContextArcMutex {
        self.ctx_ref.clone()
    }

    fn make_renderer(&'a self) -> DevinitResult<RendererVariant> {
        Ok(FileRenderer::new(&self)?)
    }
}

/// A template to initialise a project (i.e. a directory of files)
#[derive(Debug, Clone)]
pub struct ProjectTemplate {
    ctx_ref: ContextArcMutex,

    name: String,
    /// Hashmap of literals.
    /// Key is the relative filename + folder structure to emit (e.g. foo/bar/baz.txt)
    /// Value is the templated file literal
    literals: HashMap<String, String>,
    source: String,

    /// Names of each template file as can be found in the Tera instance
    file_template_names: Vec<String>,
}

impl<'a> Template<'a> for ProjectTemplate {
    type Me = Self;

    /// Load the project template from a configuration file in addition to any template configuration scripts
    fn load<P: AsRef<Path>>(path: P, ctx: ContextArcMutex) -> DevinitResult<Self::Me> {
        let proj_dir = path
            .as_ref()
            .parent()
            .ok_or(DevinitError::MissingProjectDirError(
                path.as_ref().display().to_string(),
            ))?;

        let cfg_builder = ProjectTemplateYamlBuilder::new(proj_dir.join("templaterc.yml"))?;
        let cfg = cfg_builder.build()?;

        let name = cfg_builder.name();

        // load each referenced file in the project template as a literal
        let mut file_template_names = vec![];
        let mut literals = HashMap::new();
        for (k, v) in cfg.files {
            // try to load `v`, which will render to output path `k`.
            let lit = fs::read_to_string(cfg_builder.folder().join(&v))
                .map_err(|e| DevinitError::FileReadWriteError(e.to_string()))?;
            literals.insert(k.clone(), lit.clone());

            let id = format!("{}/{}", &name, &k);

            let mut ctx_lock = ctx.lock().unwrap();
            ctx_lock
                .tera_mut()
                .add_raw_template(&id.as_str(), &lit)
                .into_diagnostic()
                .map_err(|e| DevinitError::TemplateParseError(format!("{:?}", e)))?;

            file_template_names.push(id);
        }

        Ok(Self {
            ctx_ref: ctx.clone(),
            name: cfg_builder.name().clone(),
            literals,
            source: path.as_ref().display().to_string(),
            file_template_names,
        })
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn context(&self) -> ContextArcMutex {
        self.ctx_ref.clone()
    }

    fn make_renderer(&'a self) -> DevinitResult<RendererVariant> {
        Ok(ProjectRenderer::new(&self)?)
    }
}

impl ProjectTemplate {
    pub fn file_template_names(&self) -> &Vec<String> {
        &self.file_template_names
    }

    pub fn literals(&self) -> &HashMap<String, String> {
        &self.literals
    }
}

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
