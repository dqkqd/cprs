use crate::utils::println_to_console;

use super::{cmd::Init, Run};
use anyhow::Result;

impl Run for Init {
    fn run(&self) -> Result<()> {
        let function = r#"
cprs() {
  if [[ "$#" -eq 2 && "$1" == "cd" ]]
  then
    result=$(cprs_cli "$@")
    echo "Change directory to $result"
    cd $result
  else
    cprs_cli "$@"
  fi
}
                "#;
        println_to_console(function);
        Ok(())
    }
}
