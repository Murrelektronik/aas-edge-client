use tokio::process::Command;
use anyhow:: anyhow;

/// Executes a bash script and returns its output as a String.
pub async fn run_bash_script(script_path: &str) -> anyhow::Result<String> {
    let output = Command::new("bash")
        .arg(script_path)
        .output()
        .await
        .map_err(|e| anyhow!("Failed to execute script: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Script execution failed: {}", stderr))
    }
}