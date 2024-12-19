use crate::interface::read_dir_files;
use std::error::Error;
use std::path::PathBuf;

/// Describes the type of template
#[derive(Debug)]
pub enum TemplateType {
    Header,
    Footer,
}

/// Holds information regarding a template
#[derive(Debug)]
pub struct Template {
    kind: TemplateType,
    path: PathBuf,
}
impl Template {
    pub fn get_kind(&self) -> &TemplateType {
        &self.kind
    }
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

/// Extracts template file info from cfg directory found next to the executable
pub fn get_templates(extension_type: &str) -> Result<Vec<Template>, Box<dyn Error>> {
    let cfg_path: PathBuf = PathBuf::from("cfg");
    let cfg_files = read_dir_files(&cfg_path, extension_type, false)?;
    let mut templates: Vec<Template> = Vec::new();
    for entry in cfg_files {
        let name: &str = entry.file_name().unwrap().to_str().unwrap();
        if name.starts_with("header") {
            templates.push(Template {
                kind: TemplateType::Header,
                path: entry,
            });
        } else if name.starts_with("footer") {
            templates.push(Template {
                kind: TemplateType::Footer,
                path: entry,
            });
        }
    }
    Ok(templates)
}
