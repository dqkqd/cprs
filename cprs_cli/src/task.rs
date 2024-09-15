use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{config::Config, history::History, utils::println_to_console};

// https://github.com/jmerle/competitive-companion?tab=readme-ov-file#the-format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    group: String,
    url: String,
    interactive: bool,
    memory_limit: u16,
    time_limit: u16,
    tests: Vec<TestCase>,
    test_type: TestType,
    languages: Languages,
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for Task {}

impl Task {
    pub fn metadata_file(&self) -> PathBuf {
        self.task_folder().unwrap().join("task_desc.json")
    }
    pub fn summary(&self) -> String {
        format!("Task `{}`, from `{}`", &self.name, &self.url)
    }
    pub fn task_folder(&self) -> anyhow::Result<PathBuf> {
        let normalize = |s: &str| s.trim().replace("  ", " ").replace(" ", "_").to_lowercase();
        let mut iter = self.group.split('-').map(normalize);
        let site = iter
            .next()
            .with_context(|| format!("Cannot get contest site from {}", &self.group))?;
        let contest_name = iter
            .next()
            .with_context(|| format!("Cannot get contest name from {}", &self.group))?;
        let task_name = match &self.languages {
            Languages::Java(lang) => &lang.task_class,
        };
        Ok(Config::load()
            .workspace
            .join(site)
            .join(contest_name)
            .join(task_name))
    }
    pub fn setup(&self) -> anyhow::Result<()> {
        let task_folder = self.task_folder()?;
        let test_folder = task_folder.join("tests");
        fs::create_dir_all(&test_folder)?;
        for (i, test_case) in self.tests.iter().enumerate() {
            let case_name = format!("case_{:02}", i + 1);
            let input = test_folder.join(format!("{case_name}.in"));
            let output = test_folder.join(format!("{case_name}.out"));
            fs::write(input, &test_case.input)?;
            fs::write(output, &test_case.output)?;
        }
        fs::write(self.metadata_file(), serde_json::to_string(&self)?)?;
        println_to_console(format!("Task created: `{}`", task_folder.display()));
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
