use budget_manager::budgeting::budget::Budget;
use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use clap::{Arg, App};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_new_budget() -> Result<(), Box<dyn std::error::Error>>{
        let mut cmd = Command::cargo_bin("budget_manager")?;
        cmd.arg("spent")
            .args(["--amount=2000", "--category=Bills"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Transaction added: BDT2000 @ Bills"));
        Ok(())
    }
}
