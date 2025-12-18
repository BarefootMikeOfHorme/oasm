use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub payload: String,
}

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn send_msg_fnc(id: u32, payload: *const u8, len: usize) -> bool {
    let slice = std::slice::from_raw_parts(payload, len);
    let msg = Message {
        id,
        payload: String::from_utf8_lossy(slice).to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    info!("send_msg_fnc: {}", json);
    true
}
