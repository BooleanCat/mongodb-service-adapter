extern crate tempdir;

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use std::process::Command;

    #[test]
    fn it_works() {
        let temp_dir = TempDir::new("")
            .expect("failed to create tempdir");

        let bin_path = temp_dir.path().join("mongodb-service-adapter");
        let command = &mut Command::new("rustc");
        let command = command
            .arg("-o")
            .arg(bin_path)
            .arg("src/main.rs");

        let _ = command.status()
            .expect("failed to build");

        let adapter_command = &mut Command::new(temp_dir.path().join("mongodb-service-adapter"));
        let adapter_command = adapter_command.arg("generate-manifest");

        let status = adapter_command.status()
            .expect("failed to build");

        let exit_code = status.code().unwrap();

        assert_eq!(exit_code, 10);
    }
}
