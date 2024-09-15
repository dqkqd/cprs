use std::{fs, path::PathBuf};

use anyhow::Context;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{config::Config, history::History, template::Template, utils::println_to_console};

// https://github.com/jmerle/competitive-companion?tab=readme-ov-file#the-format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRaw {
    pub name: String,
    pub group: String,
    pub url: String,
    interactive: bool,
    memory_limit: u16,
    time_limit: u16,
    tests: Vec<TestCase>,
    test_type: TestType,
    languages: Languages,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub raw: TaskRaw,
    pub config: Config,
    pub task_name: String,
    pub task_folder: PathBuf,
}

impl From<TaskRaw> for Task {
    fn from(task_raw: TaskRaw) -> Self {
        let raw = task_raw;
        let config = Config::load();
        let task_name = match &raw.languages {
            Languages::Java(lang) => &lang.task_class,
        }
        .to_snake_case();

        let normalize = |s: &str| s.trim().to_snake_case();
        let mut iter = raw.group.split('-').map(normalize);
        let contest_site = iter
            .next()
            .with_context(|| format!("Cannot get contest site from {}", &raw.group))
            .unwrap();
        let contest_name = iter
            .next()
            .with_context(|| format!("Cannot get contest name from {}", &raw.group)).unwrap();
        let task_folder = config.workspace.join(contest_site).join(contest_name).join(&task_name);

        Self {
            raw,
            config,
            task_name,
            task_folder,
        }
    }
}

impl Task {
    pub fn summary(&self) -> String {
        format!("Task `{}`, from `{}`", &self.raw.name, &self.raw.url)
    }
    pub fn setup(&self) -> anyhow::Result<()> {
        self.setup_testcases()?;
        self.setup_templates()?;
        self.setup_metadata()?;
        println_to_console(format!("Task created: {}", self.summary()));
        Ok(())
    }
    fn setup_testcases(&self) -> anyhow::Result<()> {
        let test_folder = self.task_folder.join("tests");
        fs::create_dir_all(&test_folder)?;
        for (i, test_case) in self.raw.tests.iter().enumerate() {
            let case_name = format!("case_{:02}", i + 1);
            let input = test_folder.join(format!("{case_name}.in"));
            let output = test_folder.join(format!("{case_name}.out"));
            fs::write(input, &test_case.input)?;
            fs::write(output, &test_case.output)?;
        }
        Ok(())
    }
    fn setup_templates(&self) -> anyhow::Result<()> {
        fs::create_dir_all(self.task_folder.join("src"))?;

        let rendered_cargo = Template::render_cargo(self)?;
        let cargo_file = self.task_folder.join("Cargo.toml");
        fs::write(cargo_file, rendered_cargo)?;

        let rendered_main = Template::render_main(self)?;
        let main_file = self.task_folder.join("src").join("main.rs");
        fs::write(main_file, rendered_main)?;

        Ok(())
    }
    fn setup_metadata(&self) -> anyhow::Result<()> {
        let metadata_file = self.task_folder.join("task_desc.json");
        fs::write(metadata_file, serde_json::to_string(&self.raw)?)?;
        History::add_task(self.clone())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum TestType {
    Single,
    MultiNumber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Languages {
    Java(JavaLang),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JavaLang {
    main_class: String,
    task_class: String,
}
