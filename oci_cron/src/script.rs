use crate::model::OciCreds;
use std::env;
use std::process::Command;

impl OciCreds {
    pub fn from_env() -> Self {
        Self {
            auth: env::var("AUTH").expect("AUTH missing"),
            profile: env::var("PROFILE").expect("PROFILE missing"),
            availability_domain: env::var("AVAI_DOMAIN").expect("AVAI_DOMAIN missing"),
            compartment_id: env::var("TENANCY_ID").expect("TENANCY_ID missing"),
            shape: env::var("SHAPE").unwrap_or_else(|_| "VM.Standard.A1.Flex".to_string()),

            shape_config: serde_json::from_str(
                &env::var("SHAPE_CONFIG").expect("SHAPE_CONFIG missing"),
            )
            .expect("Invalid JSON in SHAPE_CONFIG"),

            image_id: env::var("IMAGE_ID").expect("IMAGE_ID missing"),
            subnet_id: env::var("SUBNET_ID").expect("SUBNET_ID missing"),

            assign_public_ip: env::var("ASSIGN_PUB_IP")
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(true),

            ssh_authorized_keys_file: env::var("SSH_AUTH_KEYS").expect("SSH_AUTH_KEYS missing"),
            display_name: env::var("DISPLAY_NAME").unwrap_or_else(|_| "bigvmlas".to_string()),
        }
    }
}

pub fn get_vm(creds: &OciCreds) -> std::io::Result<std::process::ExitStatus> {
    Command::new("oci")
        .arg("compute")
        .arg("instance")
        .arg("launch")
        .arg("--auth")
        .arg(&creds.auth)
        .arg("--profile")
        .arg(&creds.profile)
        .arg("--availability-domain")
        .arg(&creds.availability_domain)
        .arg("--compartment-id")
        .arg(&creds.compartment_id)
        .arg("--shape")
        .arg(&creds.shape)
        .arg("--shape-config")
        .arg(&creds.shape_config.to_string())
        .arg("--image-id")
        .arg(&creds.image_id)
        .arg("--subnet-id")
        .arg(&creds.subnet_id)
        .arg("--assign-public-ip")
        .arg(&creds.assign_public_ip.to_string())
        .arg("--ssh-authorized-keys-file")
        .arg(&creds.ssh_authorized_keys_file)
        .arg("--display-name")
        .arg(&creds.display_name)
        .status()
}
