use anyhow::Result;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn run_pandoc_with_defaults(
    base_dir: &str,
    args: Vec<&str>,
    defaults: &str,
) -> Result<Vec<u8>> {
    let (file, config_path) = NamedTempFile::new()?.keep()?;
    let config_path = format!("{}", config_path.display());
    log::info!("writing pandoc defaults: {}", &config_path);
    write!(&file, "{}", defaults)?;
    let mut args = args;
    args.extend(vec!["-d", config_path.as_ref()]);
    run_pandoc(base_dir, args)
}

pub fn run_pandoc(base_dir: &str, args: Vec<&str>) -> Result<Vec<u8>> {
    let output = Command::new("pandoc")
        .current_dir(base_dir)
        .args(args)
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    for line in stderr.lines() {
        log::warn!("[pandoc] {}", line);
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("pandoc failure: {}", stderr));
    }

    Ok(output.stdout)
}

pub fn run_pandoc_citeproc(base_dir: &str, bibfile_path: &str) -> Result<String> {
    log::info!("running: pandoc-citeproc -y {}", bibfile_path);
    let output = Command::new("pandoc-citeproc")
        .current_dir(base_dir)
        .arg("-y")
        .arg(bibfile_path)
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    for line in stderr.lines() {
        log::warn!("[pandoc] {}", line);
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("pandoc failure: {}", stderr));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_run_pandoc() {
        init();
        let base_dir = "test/content/markdown";
        let config = r###"
        input-file: content.md
        "###;

        let result = run_pandoc_with_defaults(base_dir, vec![], config).expect("failed to call pandoc");
        let result = String::from_utf8(result).expect("invalid utf-8 data");
        assert!(result.contains("I love caramels tootsie roll"));
    }

    #[test]
    pub fn test_run_citeproc() {
        init();
        let result = run_pandoc_citeproc("test/content/", "references.bib")
            .expect("failed to call pandoc-citeproc");
        let substr = "Impartial Triangular Chocolate Bar Games".to_lowercase();
        assert!(
            result.to_lowercase().contains(&substr),
            "{} did not contain '{}'",
            &result,
            &substr,
        );
    }

}
