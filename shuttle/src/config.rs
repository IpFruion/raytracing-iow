use envconfig::Envconfig;

const SHUTTLE_URL: &str = "https://raytracing-iow.shuttleapp.rs";
const LOCAL_URL: &str = "http://localhost:8000";

#[derive(Clone, Envconfig, Debug)]
pub struct AppConfig {
    #[envconfig(from = "SHUTTLE", default = "false")]
    pub shuttle: bool,
}

impl AppConfig {
    pub fn root_url(&self) -> &'static str {
        if self.shuttle {
            SHUTTLE_URL
        } else {
            LOCAL_URL
        }
    }
}
