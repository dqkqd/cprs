use std::fs;

use anyhow::Context;
use minijinja::{context, path_loader, Environment};

use crate::{config::Config, task::Task};

pub struct Template {}

impl Template {
    pub fn render_cargo(task: &Task) -> anyhow::Result<String> {
        let config = Config::load();
        let algo_path_relative = pathdiff::diff_paths(&config.algo_lib, task.task_folder()?)
            .with_context(|| "Cannot find different between algo lib path and task folder")?;

        let mut env = Environment::new();
        env.set_loader(path_loader(&config.templates));

        let template = env.get_template("template_Cargo.toml")?;
        let rendered = template.render(context! {
            task_name => task.task_name(),
            algo_lib => algo_path_relative,
        })?;
        Ok(rendered)
    }

    pub fn render_main(_task: &Task) -> anyhow::Result<String> {
        let config = Config::load();
        let rendered = fs::read_to_string(config.templates.join("template_main.rs"))?;
        Ok(rendered)
    }
}
