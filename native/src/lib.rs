#[macro_use]
extern crate neon;

use neon::prelude::*;

fn bcc(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;

    let vec_of_numbers: Vec<f64> = vec
        .iter()
        .map(|js_value| {
            js_value.downcast::<JsNumber>().unwrap_or(cx.number(0)).value()
        })
        .collect();

    let mut calculated_bcc: u8 = 0;

    for byte in vec_of_numbers {
        let mut b: u8 = byte.round() as u8;
        calculated_bcc ^= b;
    }

    Ok(cx.number(calculated_bcc))
}

register_module!(mut cx, {
    cx.export_function("bcc", bcc)?;
    Ok(())
});
