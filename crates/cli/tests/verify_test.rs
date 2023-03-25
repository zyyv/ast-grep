mod common;

use anyhow::Result;
use ast_grep::main_with_args;
use common::create_test_files;
use tempdir::TempDir;

const CONFIG: &str = "
ruleDirs:
- rules
testConfigs:
- testDir: rule-test
";
const RULE: &str = "
id: test-rule
message: test rule
severity: warning
language: TypeScript
rule:
  pattern: Some($A)
";

const TEST: &str = "
id: test-rule
valid:
- None
invalid:
- Some(123)
";

fn setup() -> TempDir {
  let dir = create_test_files([
    ("sgconfig.yml", CONFIG),
    ("rules/test-rule.yml", RULE),
    ("rule-test/test-rule-test.yml", TEST),
    ("test.ts", "Some(123)"),
  ]);
  assert!(dir.path().join("sgconfig.yml").exists());
  dir
}

fn sg(s: &str) -> Result<()> {
  println!("{}", s);
  let args = s.split(' ').map(String::from);
  main_with_args(args)
}

#[test]
fn test_sg_test() {
  let dir = setup();
  let config = dir.path().join("sgconfig.yml");
  let ret = sg(&format!(
    "sg test -c {} --skip-snapshot-tests",
    config.display()
  ));
  assert!(ret.is_ok());
  drop(dir);
}