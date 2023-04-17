#[derive(
    Debug, 
    PartialEq, 
    serde::Serialize, 
    serde::Deserialize
)]
#[serde(default)]
pub struct SApp {
    pub bind_ip: String,
    pub port: u16,
    pub threads: usize,
    pub header_prefix: String,
    pub public: String,
    pub size_limit: usize
}

#[derive(
    Debug, 
    PartialEq, 
    serde::Serialize, 
    serde::Deserialize
)]
#[serde(default)]

pub struct Config {
    pub se_app: SApp,
}

impl Config {
    /// Generate the config file.
    /// Values taken from default.
    /// # Arguments
    /// * `cfg_path` - Path of the config
    async fn generate(
        cfg_path: &str
    ) 

    {
        let data = Self::default();

        tokio::fs::write(
            cfg_path, 
            serde_json::to_string_pretty(&data).unwrap()
        )
            .await
            .expect("Config generation failed!");
    }
    /// Load a file from a given path
    /// # Arguments
    /// * `cfg_path` - Path to load the config file from
    pub async fn load(
        cfg_path: &str
    ) -> 
        Self 
    {
        if !std::path::Path::new(
            cfg_path
        ).exists(

        ) {
            println!("Config file not found! Generating config file!");
            Self::generate(
                cfg_path
            ).await;
        }

        let cfg: Config =
            match serde_json::from_str(
                tokio::fs::read_to_string(
                    cfg_path
                ).await.unwrap().as_str()
            ) {
                Ok(
                    conf
                ) => {
                    println!("Config valid!");
                    conf
                }

                Err(
                    err
                ) => {
                    println!("Invalid config!");
                    println!("{:?}", err);
                    std::process::exit(1)
                }
            };
        tokio::fs::write(
            cfg_path, 
            serde_json::to_string_pretty(
                &cfg
            ).unwrap()
        )
            .await
            .expect("Config write failed! Write permissions required!");
        cfg
    }
}

impl Default for Config {
    /// Defaults for the config file.
    /// Used for the generation of the config file.
    fn default(
        
    ) -> 
        Self 
    {
        Config {
            se_app: SApp::default(),
        }
    }
}

impl Default for SApp {
    fn default(

    ) -> 
        Self 
    {
        SApp {
            bind_ip: "0.0.0.0".to_owned(),
            port: 8443,
            threads: std::thread::available_parallelism().unwrap().get(),
            header_prefix: "X-Bare-".to_owned(),
            public: "public/".to_owned(),
            size_limit: 905_318_008
        }
    }
}
