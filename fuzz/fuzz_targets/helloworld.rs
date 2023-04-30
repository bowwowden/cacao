#![no_main]
use libfuzzer_sys::fuzz_target;
// use cacao::appkit::{App, AppDelegate};
// use cacao::appkit::window::Window;


// TODO: add docs about arbitrary
fuzz_target!(|value: &[u8]| {
    // TODO: add docs about how to fuzz
    
    // #[derive(Default)]
    // struct BasicApp {
    //     window: Window
    // }
    
    // impl AppDelegate for BasicApp {
    //     fn did_finish_launching(&self) {
    //        self.window.set_minimum_content_size(400., 400.);
    //        self.window.set_title("Hello World!");
    //        self.window.show();
    //     }
    // }
    
    // fn main() {
    //     App::new("com.hello.world", BasicApp::default()).run();
    // }

});