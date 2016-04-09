extern crate config;

use std::path::Path;
use self::config::reader;
use self::config::types::Value;
use self::config::types::ScalarValue;

pub struct Settings {
    pub moves_per_second: u32,
    pub world_size: u32
}

impl Settings {
    pub fn load() -> Result<Settings, &'static str> {
        let path = Path::new("resources/application.conf");
        let cfg = reader::from_file(path);

        match cfg {
            Ok(cfg) => {
                let mut world_size = 100;
                if let Some(&Value::Svalue(ScalarValue::Integer32(size))) = cfg.lookup("application.world.size") {
                    world_size = size
                }

                let mut world_mps = 100;
                if let Some(&Value::Svalue(ScalarValue::Integer32(mps))) = cfg.lookup("application.world.moves_per_second") {
                    world_mps = mps
                }

                Ok(Settings {
                    world_size: world_size as u32,
                    moves_per_second: world_mps as u32
                })
            },
            Err(e) => Err(e.desc)
        }
    }
}
