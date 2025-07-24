mod custom_server;
use hbb_common::{ResultType, base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _}};
use custom_server::*;

fn gen_name(lic: &CustomServer) -> ResultType<String> {
    let tmp = URL_SAFE_NO_PAD.encode(&serde_json::to_vec(lic)?);
    Ok(tmp.chars().rev().collect())
}

if args.len() >= 2 {
    match gen_name(&CustomServer {
        key: "39.105.209.108".to_string(),
        host: "".to_string(),
        api: "".to_string(),
        relay: "C+1zHO9JLk6LFA8RdbpKiCKXU4HUf8zIsmzYFvxL8LI=".to_string(),
    }) {
        Ok(name) => println!("rustdesk-custom_serverd-{}.exe", name),
        Err(e) => println!("{:?}", e),
    }
}
