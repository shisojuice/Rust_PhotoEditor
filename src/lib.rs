
use wasm_bindgen::prelude::*;
use std::cell::{Cell, RefCell};
use std::io::Cursor;
use std::rc::Rc;
use image::{DynamicImage, ImageFormat};
use web_sys::{Blob, BlobPropertyBag, ImageData, Url};
use web_sys::js_sys::{Array, Uint8Array};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn img_canvas(image_data: &[u8]) {
    let img = image::load_from_memory(image_data).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_id("myCanvas");
    document.body().unwrap().append_child(&canvas).unwrap();
    canvas.set_width(img.width());
    canvas.set_height(img.height());
    canvas.style().set_property("border", "solid").unwrap();
    let context =
        canvas.get_context("2d").unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    let context = Rc::new(context);

    // image crate の画像データを RGBA のベクタに変換
    let raw_pixels: Vec<u8> = match img {
        DynamicImage::ImageRgba8(ref img_buffer) => img_buffer.clone().into_raw(),
        _ => {
            // 他のフォーマットの場合は RGBA に変換する
            img.to_rgba8().into_raw()
        }
    };
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut raw_pixels.clone()),
        img.width(),
        img.height(),
    ).unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();

    context.set_line_width(3f64);
    let pressed = Rc::new(Cell::new(false));
    #[derive(Clone)]
    struct Point {
        x: f64,
        y: f64,
    }
    // Rc<RefCell<Vec<Point>>> を使用してベクターを共有する
    let my_xy = Rc::new(RefCell::new(Vec::new()));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let canvas_clone = canvas.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            // Canvas要素の位置を取得
            let rect = canvas_clone.get_bounding_client_rect();
            let mouse_x = event.client_x() as f64 - rect.left();
            let mouse_y = event.client_y() as f64 - rect.top();
            //Canvas外
            if mouse_x < 0f64|| mouse_x > canvas_clone.width() as f64 || mouse_y < 0f64 || mouse_y > canvas_clone.height() as f64 {
                pressed.set(false);
            }
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    {
        // smartphone_event
        let context = context.clone();
        let canvas_clone = canvas.clone();
        let my_xy_clone = my_xy.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            // Canvas要素の位置を取得
            let rect = canvas_clone.get_bounding_client_rect();
            // タッチ座標をCanvas座標系に変換
            let canvas_x = event.touches().get(0).unwrap().client_x() as f64 - rect.left();
            let canvas_y = event.touches().get(0).unwrap().client_y() as f64 - rect.top();
            // 現在の位置に矩形を描画
            context.fill_rect(canvas_x, canvas_y, 1f64, 1f64);
            my_xy_clone.borrow_mut().push(Point{x:canvas_x, y:canvas_y});
        });
        canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    {
        // smartphone_event
        let context = context.clone();
        let my_xy_clone = my_xy.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::TouchEvent| {
            for i in 0..my_xy_clone.borrow().len(){
                if i > 0 && i < my_xy_clone.borrow().len() - 1 {
                    context.begin_path();
                    context.move_to(my_xy_clone.borrow().get(i-1).unwrap().x,my_xy_clone.borrow().get(i-1).unwrap().y);
                    context.line_to(my_xy_clone.borrow().get(i).unwrap().x,my_xy_clone.borrow().get(i).unwrap().y);
                    context.stroke();
                }
            }
            my_xy_clone.borrow_mut().clear();
        });
        canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

#[wasm_bindgen]
pub fn dl_canvas(image_data: &[u8]) {
    let img = image::load_from_memory(image_data).unwrap();

    //DL
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
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