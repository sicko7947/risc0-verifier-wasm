use wasm_bindgen::prelude::*;
use risc0_zkvm::{compute_image_id, Receipt, Digest};

// Helper to convert errors to JsValue for wasm-bindgen
fn to_js_error(err_msg: String) -> JsValue {
    js_sys::Error::new(&err_msg).into()
}

#[wasm_bindgen]
pub fn verify_receipt_wasm(receipt_bytes: &[u8], image_bytes: &[u8]) -> Result<Box<[u8]>, JsValue> {
    console_error_panic_hook::set_once(); // Better panic messages in browser console

    let receipt: Receipt = bincode::deserialize(receipt_bytes)
        .map_err(|e| to_js_error(format!("Failed to deserialize receipt: {}", e)))?;

    let image_id: Digest = compute_image_id(image_bytes)
        .map_err(|e| to_js_error(format!("Failed to compute image ID: {}", e)))?;

    receipt.verify(image_id.clone()) 
        .map_err(|e| to_js_error(format!("Receipt verification failed: {:?}", e)))?;

    // Return the raw journal bytes
    let journal_bytes = receipt.journal.bytes.clone();
    Ok(journal_bytes.into_boxed_slice())
}
