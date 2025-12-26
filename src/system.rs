use crate::config::Config;
use crate::resources::{PackagesResource, ServicesResource};



pub struct SystemState {
    pub installed_packages: Vec<String>,
    pub enabled_services: Vec<String>,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            installed_packages: PackagesResource::list_installed(),
            enabled_services: ServicesResource::list_enabled(),
        }
    }

    pub fn diff(&self, cfg: &Config) {
        if let Some(packages) = &cfg.packages {
            if let Some(to_install) = &packages.install {
                for pkg in to_install {
                    if !self.installed_packages.contains(pkg) {
                        println!("+ install {}", pkg);
                    } else {
                        println!("= {} already installed", pkg);
                    }
                }
            }
        }

        if let Some(services) = &cfg.services {
            if let Some(to_enable) = &services.enable {
                for svc in to_enable {
                    if !self.enabled_services.contains(svc) {
                        println!("+ enable {}", svc);
                    } else {
                        println!("= {} already enabled", svc);
                    }
                }
            }
        }
    }

    pub fn apply(&self, cfg: &Config, dry_run: bool) {
        if let Some(packages) = &cfg.packages {
            if let Some(to_install) = &packages.install {
                for pkg in to_install {
                    if !self.installed_packages.contains(pkg) {
                        PackagesResource::apply(pkg, dry_run);
                    }
                }
            }
        }

        if let Some(services) = &cfg.services {
            if let Some(to_enable) = &services.enable {
                for svc in to_enable {
                    if !self.enabled_services.contains(svc) {
                        ServicesResource::apply(svc, dry_run);
                    }
                }
            }
        }
    }
}
