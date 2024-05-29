/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{collections::HashMap, fs, path::Path};

use miette::IntoDiagnostic;

use crate::{
    error::{DevinitError, DevinitResult},
    files::ProjectTemplateYamlBuilder,
    templater::{ContextArcMutex, ProjectRenderer, Renderer, RendererVariant},
};

use super::Template;

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

// Similar to file templates, we just compare project templates by name:
impl Eq for ProjectTemplate {}
impl PartialEq for ProjectTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Ord for ProjectTemplate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for ProjectTemplate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
