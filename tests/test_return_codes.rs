extern crate tempdir;

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn generate_manifest_is_not_implemented() {
        let temp_dir = setup();
        let exit_code = run_adapter(temp_dir.path(), "generate-manifest");

        assert_eq!(exit_code, 10);
    }

    #[test]
    fn create_binding_is_not_implemented() {
        let temp_dir = setup();
        let exit_code = run_adapter(temp_dir.path(), "create-binding");

        assert_eq!(exit_code, 10);
    }

    #[test]
    fn delete_binding_is_not_implemented() {
        let temp_dir = setup();
        let exit_code = run_adapter(temp_dir.path(), "delete-binding");

        assert_eq!(exit_code, 10);
    }

    #[test]
    fn dashboard_url_is_not_implemented() {
        let temp_dir = setup();
        let exit_code = run_adapter(temp_dir.path(), "dashboard-url");

        assert_eq!(exit_code, 10);
    }

    #[test]
    fn returns_1_when_called_with_invalid_arg() {
        let temp_dir = setup();
        let exit_code = run_adapter(temp_dir.path(), "foo");

        assert_eq!(exit_code, 1);
    }

    fn setup() -> TempDir {
        let temp_dir = TempDir::new("").expect("failed to create tempdir");
        build_adapter(temp_dir.path());
        temp_dir
    }

    fn build_adapter(path: &Path) {
        let bin_path = path.join("mongodb-service-adapter");
        let command = &mut Command::new("rustc");
        let command = command.args(&["-o", bin_path.to_str().unwrap(), "src/main.rs"]);

        command.status().expect("failed to build");
    }

    fn run_adapter(path: &Path, arg: &str) -> i32 {
        let adapter_command = &mut Command::new(path.join("mongodb-service-adapter"));
        let adapter_command = adapter_command.arg(arg);
        let status = adapter_command.status().expect("failed to build");
        status.code().expect("failed to get exit code")
    }
}
