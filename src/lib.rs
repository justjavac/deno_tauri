extern crate deno_core;

use deno_core::plugin_api::{Buf, Interface, Op, ZeroCopyBuf};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("op_tauri", op_tauri);
}

fn op_tauri(
  _interface: &mut dyn Interface,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let result = b"wip";
  let result_box: Buf = Box::new(*result);
  Op::Sync(result_box)
}
