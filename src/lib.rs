
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::CssStyleSheet;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let root = get_root_element()?;

    let app = smithy::smd!(
        <div>
            <div class="blue">Hello from wasm</div>
        </div>
      );

    smithy::mount(Box::new(app), root);
    console::log_1(&JsValue::from_str("Hello Alfin!"));

    Ok(())
}

fn get_root_element() -> Result<web_sys::Element, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id("app")
        .ok_or(JsValue::NULL)
}

