#[macro_use]
extern crate neon;
extern crate image;
extern crate base64;

use neon::prelude::*;

fn raw_to_jpeg(mut cx: FunctionContext) -> JsResult<JsString> {
	let base64_raw_img = cx.argument::<JsString>(0)?.value();
	let imgx = cx.argument::<JsNumber>(1)?.value();
	let imgy = cx.argument::<JsNumber>(2)?.value();

    let raw_img_vec = base64::decode(&base64_raw_img).unwrap();

    let dyn_image = image::load_from_memory(&raw_img_vec[..]).unwrap();
    let raw_img = dyn_image.into_rgb().into_raw();

    let mut jpg_vec: Vec<u8> = Vec::new();
    let quality = 0; // 0 - worst, 100 - best
    let mut encoder = image::jpeg::JPEGEncoder::new_with_quality(&mut jpg_vec, quality);

	encoder.encode(&raw_img, imgx as u32, imgy as u32, image::ColorType::RGB(8)); 
	let base64_jpg = base64::encode(&jpg_vec[..]);

	Ok(cx.string(base64_jpg))
}

register_module!(mut m, {
    m.export_function("rawToJpeg", raw_to_jpeg)
});




