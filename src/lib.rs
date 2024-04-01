use std::collections::HashMap;

use lazy_static::lazy_static;
use shadowsocks_service::{
    config::{Config, ConfigType}, run_local
};
use tokio::task;
use std::sync::Mutex;

lazy_static! {
 static  ref MAP: Mutex<HashMap<i32, task::JoinHandle<Result<(), std::io::Error>>>> = {
        let map:HashMap<i32, task::JoinHandle<Result<(), std::io::Error>>> = HashMap::new();
         Mutex::new(map)
        };
}
lazy_static!{
    static ref RUNNER: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
}
#[no_mangle]
pub extern "C" fn stopLocalHTTPProxyByID(id:i32) {
    println!("rust:stopLocalHTTPPRoxyByID ID={}",id);
    MAP.lock().unwrap().get_mut(&id).unwrap().abort();
    MAP.lock().unwrap().remove(&id).unwrap();
}
#[no_mangle]
pub extern "C" fn startLocalHTTPProxyServiceByID(
    id :i32,
    local_address_str:*const u8,
    local_address_str_len:usize,
    local_port:i32,
    server_address_str:*const u8,
    server_address_str_len:usize,
    server_port:i32,
    passwd_str:*const u8,
    passwd_str_len:usize){
    let local_address = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(local_address_str, local_address_str_len)
        )
    };
    let server_address = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(server_address_str, server_address_str_len)
        )
    };
    let passwd = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(passwd_str, passwd_str_len)
        )
    };
    let config = format!(r#"{{
        "locals": [
            {{
                "local_port": {},
                "local_address": "{}",
                "protocol": "http"
            }}
        ],
        "server": "{}",
        "server_port": {},
        "password": "{}",
        "method": "aes-128-gcm"
    }}"#,local_port,local_address,server_address,server_port,passwd);
    println!("rust:startLocalHTTPPoxyServiceByID ID={} local_addr={}:{} server_addr={}:{} passwd={} "
           ,id,
           local_address,local_port,
           server_address,server_port,
           passwd);
    let local_config = Config::load_from_str(config.as_str()
                                             , ConfigType::Local).unwrap();
    let job = RUNNER.spawn(run_local(local_config));
    MAP.lock().unwrap().insert(id,job);
    RUNNER.block_on(async {
        while MAP.lock().unwrap().contains_key(&id) {
            println!("rust: ID={} is Running...",id);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        println!("rust: ID={} Exit OK",id);
    });
}