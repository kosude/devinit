/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{ffi::OsStr, fs, path::Path};

use miette::IntoDiagnostic;

use crate::{
    error::{DevinitError, DevinitResult},
    templater::{ContextArcMutex, FileRenderer, Renderer, RendererVariant},
};

use super::Template;

/// A template to initialise a single file
#[derive(Debug, Clone)]
pub struct FileTemplate {
    ctx_ref: ContextArcMutex,

    name: String,
    source: String,
}

// When comparing file templates, we just want to compare them by name:
impl Eq for FileTemplate {}
impl PartialEq for FileTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Ord for FileTemplate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for FileTemplate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Template<'a> for FileTemplate {
    type Me = Self;

    /// Load the file template from a single template configuration script
    fn load<P: AsRef<Path>>(path: P, ctx: ContextArcMutex) -> DevinitResult<Self::Me> {
        let name = String::from(path.as_ref().file_name().and_then(OsStr::to_str).ok_or(
            DevinitError::FileReadWriteError(format!(
                "Failed to extract filename from path {:?}",
                path.as_ref()
            )),
        )?);
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
