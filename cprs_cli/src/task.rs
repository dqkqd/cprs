use std::path::PathBuf;

use anyhow::{Context, Result};
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use tokio::fs;

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
    pub src_folder: PathBuf,
    pub test_folder: PathBuf,
    pub cargo_file: PathBuf,
    pub main_file: PathBuf,
    pub submit_file: PathBuf,
    pub metadata_file: PathBuf,
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
            .with_context(|| format!("Cannot get contest name from {}", &raw.group))
            .unwrap();
        let task_folder = config
            .workspace
            .join(contest_site)
            .join(contest_name)
            .join(&task_name);

        let src_folder = task_folder.join("src");
        let test_folder = task_folder.join("tests");
        let cargo_file = task_folder.join("Cargo.toml");
        let main_file = src_folder.join("main.rs");
        let metadata_file = task_folder.join("task_desc.json");

        let bin_folder = src_folder.join("bin");
        let submit_file = bin_folder.join("submit.rs");

        Self {
            raw,
            config,
            task_name,
            task_folder,
            src_folder,
            test_folder,
            cargo_file,
            main_file,
            submit_file,
            metadata_file,
        }
    }
}

impl Task {
    pub fn summary(&self) -> String {
        format!("Task `{}`, from `{}`", &self.raw.name, &self.raw.url)
    }
    pub async fn setup(&self) -> Result<()> {
        self.setup_folders().await?;
        self.setup_testcases().await?;
        self.setup_templates().await?;
        self.setup_metadata().await?;
        println_to_console(format!("Task created: {}", self.summary()));
        Ok(())
    }
    pub async fn from_current_dir() -> Result<Task> {
        let task_description_file = std::env::current_dir()?.join("task_desc.json");
        let task_raw_description = fs::read_to_string(task_description_file)
            .await
            .with_context(|| "Cannot get task description")?;
        let task_raw: TaskRaw = serde_json::from_str(&task_raw_description)?;
        let task = Task::from(task_raw);
        Ok(task)
    }
    async fn setup_folders(&self) -> Result<()> {
        fs::create_dir_all(&self.task_folder).await?;
        fs::create_dir_all(&self.src_folder).await?;
        fs::create_dir_all(&self.test_folder).await?;
        if let Some(submit_folder) = &self.submit_file.parent() {
            fs::create_dir_all(submit_folder).await?;
        }
        Ok(())
    }
    async fn setup_testcases(&self) -> Result<()> {
        for (i, test_case) in self.raw.tests.iter().enumerate() {
            let case_name = format!("case_{:02}", i + 1);
            let input = self.test_folder.join(format!("{case_name}.in"));
            let output = self.test_folder.join(format!("{case_name}.out"));
            fs::write(input, &test_case.input).await?;
            fs::write(output, &test_case.output).await?;
        }
        Ok(())
    }
    async fn setup_templates(&self) -> Result<()> {
        fs::create_dir_all(&self.src_folder).await?;

        let rendered_cargo = Template::render_cargo(self)?;
        fs::write(&self.cargo_file, rendered_cargo).await?;

        let rendered_main = Template::render_main(self)?;
        fs::write(&self.main_file, rendered_main).await?;

        Ok(())
    }
    async fn setup_metadata(&self) -> Result<()> {
        fs::write(&self.metadata_file, serde_json::to_string(&self.raw)?).await?;
        History::add_task(self.clone()).await?;
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
