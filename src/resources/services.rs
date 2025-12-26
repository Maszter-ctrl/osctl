use std::process::Command;

pub struct ServicesResource;

impl ServicesResource {
    pub fn list_enabled() -> Vec<String> {
        let output = Command::new("launchctl").arg("list").output();
        match output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .skip(1)
                    .filter_map(|line| line.split_whitespace().nth(2).map(|s| s.to_string()))
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn apply(service: &str, dry_run: bool) {
        if dry_run {
            println!("[dry-run] Would enable service {}", service);
        } else {
            println!("Enabling service {}", service);
            let _ = Command::new("launchctl")
                .arg("load")
                .arg("-w")
                .arg(format!("/Library/LaunchDaemons/{}.plist", service))
                .status();
        }
    }
}
