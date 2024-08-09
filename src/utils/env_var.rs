use std::env;
use dotenv::dotenv;

pub fn get_env_var(key: &str) -> Result<String, env::VarError>{
    dotenv().ok();
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(e),
    }
}