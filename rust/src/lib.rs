extern crate jni_sys;

use jni_sys::jclass;
use jni_sys::JNIEnv;

//pack_age_Test means java class: 'pack.age.Test'
#[warn(non_snake_case)]
#[no_mangle]
pub extern fn Java_pack_age_Test_getInfo(jre: *mut JNIEnv, class: *const jclass) {
    println!("JRE: {:?}", jre);
    println!("CLASS: {:?}", class);

    //FFI methods needs to be executed in a unsafe block
    unsafe {
        let v = (**jre).GetVersion;
        println!("version: {:?}", v);
    }
}

#[warn(non_snake_case)]
#[no_mangle]
pub extern fn Java_pack_age_Test_helloWorld(jre: *mut JNIEnv, class: *const jclass) {
    println!("Hello World");
}