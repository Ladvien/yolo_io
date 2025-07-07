#[cfg(test)]
mod example_compile_test {
    use assert_cmd::Command;

    #[test]
    fn build_examples() {
        Command::new("cargo")
            .args(["build", "--example", "basic"])
            .assert()
            .success();

        Command::new("cargo")
            .args(["build", "--example", "programmatic"])
            .assert()
            .success();
    }
}
