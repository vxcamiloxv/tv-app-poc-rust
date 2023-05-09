mod utils;
mod videos;

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    utils::set_panic_hook();

    console::log_1(&JsValue::from_str("Rust module loaded!"));
    Ok(())
}

#[wasm_bindgen]
pub async fn fetch_videos(category: String) -> Result<JsValue, JsValue> {
    let video_data = videos::get_videos(category).await;

    // Send the `Videos` struct back to JS as an `Object`
    let result = match video_data {
        Ok(data) => JsValue::from_serde(&data.videos).unwrap(),
        Err(e) => return Err(JsValue::from(format!("{}", e))),
    };
    Ok(result)
}

#[wasm_bindgen]
pub fn get_video_images() -> JsValue {
    let video_images = videos::get_video_images().expect("error getting images");

    JsValue::from_serde(&video_images).unwrap()
}

#[no_mangle]
pub extern fn greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    use crate::greeting;
    use crate::videos::get_video_images;

    use std::ffi::{CString};
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};
    use jni::sys::{jstring, jobjectArray};

    #[no_mangle]
    pub unsafe extern fn Java_com_androidrust_RustGreetings_greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_androidrust_RustGreetings_fetchVideos(env: JNIEnv, _: JClass) -> jstring {
        let video_images = get_video_images().expect("error getting images");
        let images: Vec<String> = video_images.into_iter()
            .map(|video| video.picture)
            .collect();
        let array: jobjectArray = env
            .new_object_array(
                images.len() as i32,
                env.find_class("java/lang/String").unwrap(),
                *env.new_string("").unwrap(),
            )
            .unwrap();
        let mut i = 0;
        while i < images.len() as i32 {
            // Edit every Item of the Array to give it the values we want
            env.set_object_array_element(
                array,
                i,
                *env.new_string(images[i as usize].to_owned()).unwrap().to_owned(),
            ).expect("Could not perform set_object_array_element on array element.");
            i += 1;
        }
        array
    }
}
