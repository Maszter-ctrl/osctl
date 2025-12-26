use std::process::Command;

pub struct PackagesResource;

impl PackagesResource {
    pub fn list_installed() -> Vec<String> {
        let output = Command::new("brew").arg("list").output();
        match output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn apply(package: &str, dry_run: bool) {
        if dry_run {
            println!("[dry-run] Would install {}", package);
        } else {
            println!("Installing {}", package);
            let _ = Command::new("brew").arg("install").arg(package).status();
        }
    }
}
