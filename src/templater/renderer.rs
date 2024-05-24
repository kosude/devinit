/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::collections::HashMap;

use super::{ContextArcMutex, FileTemplate, ProjectTemplate, Template};
use crate::error::{ExecError, ExecResult};
use miette::IntoDiagnostic;
use tera::Context;

pub enum RendererVariant<'a> {
    File(FileRenderer<'a>),
    Project(ProjectRenderer<'a>),
}

/// A trait defining behaviour to render a template (i.e. either produce evaluated string output, or a project folder structure)
pub trait Renderer<'a> {
    type Template;
    type Output;

    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant>;

    fn add_variable<S: AsRef<str>>(&mut self, key: S, val: S);

    fn render(&self) -> ExecResult<Self::Output>;
}

/// A renderer for file templates
pub struct FileRenderer<'a> {
    ctx_ref: ContextArcMutex,

    template: &'a FileTemplate,
    var_context: tera::Context,
}

impl<'a> Renderer<'a> for FileRenderer<'a> {
    type Template = FileTemplate;
    type Output = String;

    /// Initialise a new renderer for the given file template
    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant> {
        Ok(RendererVariant::File(Self {
            ctx_ref: template.context(),
            template,
            var_context: Context::new(),
        }))
    }

    fn add_variable<S: AsRef<str>>(&mut self, key: S, val: S) {
        self.var_context.insert(key.as_ref(), val.as_ref());
    }

    /// Render the file, producing evaluated string output
    fn render(&self) -> ExecResult<Self::Output> {
        self.ctx_ref
            .lock()
            .unwrap()
            .tera()
            .render(&self.template.name(), &self.var_context)
            .into_diagnostic()
            .map_err(|e| ExecError::TemplateRenderError(format!("{:?}", e)))
    }
}

/// A renderer for project templates
pub struct ProjectRenderer<'a> {
    ctx_ref: ContextArcMutex,

    template: &'a ProjectTemplate,
    var_context: tera::Context,
}

impl<'a> Renderer<'a> for ProjectRenderer<'a> {
    type Template = ProjectTemplate;
    type Output = HashMap<String, String>;

    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant> {
        Ok(RendererVariant::Project(Self {
            ctx_ref: template.context(),
            template,
            var_context: Context::new(),
        }))
    }

    fn add_variable<S: AsRef<str>>(&mut self, key: S, val: S) {
        self.var_context.insert(key.as_ref(), val.as_ref());
    }

    fn render(&self) -> ExecResult<Self::Output> {
        let mut map = HashMap::new();

        for (outpath, _) in self.template.literals() {
            // render each templated file
            map.insert(
                outpath.clone(),
                self.ctx_ref
                    .lock()
                    .unwrap()
                    .tera()
                    .render(
                        format!("{}/{}", &self.template.name(), outpath).as_str(),
                        &self.var_context,
                    )
                    .into_diagnostic()
                    .map_err(|e| ExecError::TemplateRenderError(format!("{:?}", e)))?,
            );
        }

        Ok(map)
    }
}
