use crate::FrameBuffer;
use js_sys::Uint8ClampedArray;
use wasm_bindgen::{convert::IntoWasmAbi, describe::WasmDescribe};

impl IntoWasmAbi for FrameBuffer {
    type Abi = <Uint8ClampedArray as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Uint8ClampedArray::from(self.buffer.as_slice()).into_abi()
    }
}

impl WasmDescribe for FrameBuffer {
    fn describe() {
        <Uint8ClampedArray as WasmDescribe>::describe();
    }
}
