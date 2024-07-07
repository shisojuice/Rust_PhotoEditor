
use wasm_bindgen::prelude::*;
use std::io::Cursor;
use base64::Engine;
use base64::engine::general_purpose;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use image::imageops::FilterType;
use web_sys::{Blob, BlobPropertyBag, Url};
use web_sys::js_sys::{Array, Uint8Array};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    pub w: u32,
    pub h: u32,
    pub resize_w: u32,
    pub resize_h: u32,
}

pub struct MyResize {
    pub canvas: DynamicImage,
    pub resize_w: u32,
    pub resize_h: u32,
}

#[wasm_bindgen]
pub fn img_canvas(image_data: &[u8],my_size : f32)  -> JsValue{
    let img = image::load_from_memory(image_data).unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let bg_img = document
        .get_element_by_id("bgImg").unwrap()
        .dyn_into::<web_sys::HtmlImageElement>().unwrap();

    let mut buffer = Cursor::new(Vec::new());
    let my_resize = resize_image(img.clone(), my_size);
    my_resize.canvas.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let img_data = buffer.into_inner();
    let base64_data = general_purpose::STANDARD.encode(img_data);
    let data_url = format!("data:image/png;base64,{}", base64_data);
    bg_img.set_src(&*data_url);

    let my_struct = MyStruct {
        w: img.clone().width(),
        h: img.clone().height(),
        resize_w: my_resize.resize_w,
        resize_h: my_resize.resize_h,
    };
    serde_wasm_bindgen::to_value(&my_struct).unwrap()
}

// 画像のアスペクト比・操作しやすいサイズに設定・調整
fn resize_image(img: DynamicImage, my_size: f32) -> MyResize{
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // 横長の画像の場合
        (my_size, (my_size / aspect_ratio) as u32)
    } else {
        // 縦長の画像の場合
        (my_size * aspect_ratio , my_size as u32)
    };
    // リサイズ
    let resized_img = img.resize_exact(new_width as u32, new_height  as u32, FilterType::Lanczos3);
    let mut canvas = DynamicImage::new_rgba8(my_size as u32, my_size as u32);
    // 白を背景色とする
    let bkg_image= ImageBuffer::from_pixel(my_size as u32, my_size as u32, Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut canvas, &bkg_image, 0, 0);
    // リサイズ画像をキャンバスの中央に貼付
    let x = (my_size as u32- resized_img.width()) / 2;
    let y = (my_size as u32 - resized_img.height()) / 2;
    image::imageops::overlay(&mut canvas, &resized_img, x.into(), y.into());

    let my_resize = MyResize {
        canvas,
        resize_w: resized_img.width(),
        resize_h: resized_img.height(),
    };
    my_resize
}

#[wasm_bindgen]
pub fn dl_canvas(canvas_image_data: &[u8],file_image_data: &[u8]) {
    let mut file_img = image::load_from_memory(file_image_data).unwrap();
    let canvas_img = image::load_from_memory(canvas_image_data).unwrap();
    let resized_img = canvas_img.resize_exact(file_img.width() ,file_img.height() , FilterType::Lanczos3);
    image::imageops::overlay(&mut file_img, &resized_img, 0, 0);
    //DL
    let mut buffer = Cursor::new(Vec::new());
    file_img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let img_data = buffer.into_inner();
    let window = web_sys::window().unwrap();
    let uint8_array = Uint8Array::from(img_data.as_slice());
    let parts = Array::new();
    parts.push(&uint8_array);
    // Blobを作成
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts,BlobPropertyBag::new().type_("image/png")).unwrap();
    // BlobのURLを取得
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    // a要素を作成
    let link = window.document().unwrap().create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
    link.set_href(&url);
    link.set_download("canvas.png");
    link.click();
    // URLを解放
    Url::revoke_object_url(&url).unwrap();
}
