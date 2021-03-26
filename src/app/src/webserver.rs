use actix_web::{error, Error, get, web, App, HttpResponse, HttpServer};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use super::ENCLAVE_EID;
use sgx_types::*;

extern {
    fn ecall_change_srd_task(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, num: i32) -> sgx_status_t;
}

#[derive(Serialize, Deserialize)]
struct ChangeSrd {
    change: i32,
}

const MAX_SIZE: usize = 262_144;

#[get("/srd/change")]
pub async fn change_srd(mut payload: web::Payload) -> Result<HttpResponse, Error>  {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<ChangeSrd>(&body)?;
    unsafe {
        let mut sgx_status = sgx_status_t::SGX_SUCCESS;
        let retval = ecall_change_srd_task(ENCLAVE_EID, &mut sgx_status, obj.change);
        match retval {
            sgx_status_t::SGX_SUCCESS => {
                match sgx_status {
                    sgx_status_t::SGX_SUCCESS => {
                        println!("[+] Add {}GB srd task successfully, will execute later", obj.change);
                    },
                    _ => println!("[-] Add {}GB srd task failed {}", obj.change, sgx_status.as_str()),
                };
            },
            _ => println!("[-] Invoke SGX API failed {}", retval.as_str()),
        };
    }
    Ok(HttpResponse::Ok().body(r#"{"status": 200, "msg": "Add srd task successfully"}"#)) // <- send response
}

#[actix_web::main]
pub async fn start_webserver() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v0").service(change_srd))
    })
    .bind("127.0.0.1:12222")?
    .run();

    Ok(())
}
