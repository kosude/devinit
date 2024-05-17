/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::convert::TryFrom;

use super::{FileTemplate, ProjectTemplate, Template};
use crate::error::{ExecError, ExecResult};
use tera::{Context, Tera};

pub enum RendererVariant<'a> {
    File(FileRenderer<'a>),
    Project(ProjectRenderer),
}

/// A trait defining behaviour to render a template (i.e. either produce evaluated string output, or a project folder structure)
pub trait Renderer<'a> {
    type Template;
    type Output;

    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant>;
    fn render(&self) -> ExecResult<Self::Output>;
}

/// A renderer for file templates
pub struct FileRenderer<'a> {
    tera: Tera,
    template: &'a FileTemplate,
    context: Context,
}

impl<'a> Renderer<'a> for FileRenderer<'a> {
    type Template = FileTemplate;
    type Output = String;

    /// Initialise a new renderer for the given file template
    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant> {
        let name = &template.name();
        let literal = &template.literal();

        let mut tera = Tera::default();
        tera.add_raw_template(&name, &literal)
            .map_err(|e| ExecError::TemplateParseError(name.to_string(), e.to_string()))?;

        let context = Context::new();

        Ok(RendererVariant::File(Self {
            tera,
            template,
            context,
        }))
    }

    /// Render the file, producing evaluated string output
    fn render(&self) -> ExecResult<Self::Output> {
        let name = &self.template.name();
        self.tera
            .render(&name, &self.context)
            .map_err(|e| ExecError::TemplateRenderError(name.to_string(), e.to_string()))
    }
}

/// A renderer for project templates
pub struct ProjectRenderer {}

impl<'a> Renderer<'a> for ProjectRenderer {
    type Template = ProjectTemplate;
    type Output = String; // TODO: temporary, will be replaced by a struct or something

    fn new(template: &'a Self::Template) -> ExecResult<RendererVariant> {
        Ok(RendererVariant::Project(Self {}))
    }

    fn render(&self) -> ExecResult<Self::Output> {
        todo!()
    }
}
