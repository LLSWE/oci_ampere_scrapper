use serde_json::Value;

pub struct OciCreds {
    pub auth: String,
    pub profile: String,
    pub availability_domain: String,
    pub compartment_id: String,
    pub shape: String,
    pub shape_config: Value,
    pub image_id: String,
    pub subnet_id: String,
    pub assign_public_ip: bool,
    pub ssh_authorized_keys_file: String,
    pub display_name: String,
}

pub struct ResendCreds {
    pub resend_key: String,
    pub resend_from: String,
    pub resend_to: String,
}

pub struct AppConfig {
    pub oci: OciCreds,
    pub resend: ResendCreds,
}

impl AppConfig {
    pub fn load_all() -> Self {
        Self {
            oci: OciCreds::from_env(),
            resend: ResendCreds::get_resend_creds(),
        }
    }
}
