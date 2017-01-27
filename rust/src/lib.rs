#![crate_type="dylib"]
extern crate libc;

use libc::c_void;
use libc::c_int;

#[repr(C)]
pub struct JNINativeInterface {
    // Related methods and fields can be found in the jni.h file of your Java installation
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    #[warn(non_snake_case)]
    GetVersion: extern fn(env: *mut JNIEnv) -> c_int
}

//alias
pub type JNIEnv = *const JNINativeInterface;

#[warn(non_camel_case_types)]
pub type jobject = ();
#[warn(non_camel_case_types)]
pub type jclass = *const jobject;

//pack_age_Test means java class: 'pack.age.Test'
#[warn(non_snake_case)]
pub extern fn Java_pack_age_Test_getInfo(jre: *mut JNIEnv, class: *const jclass) {
    println!("JRE: {:?}", jre);
    println!("CLASS: {:?}", class);

    //FFI methods needs to be executed in a unsafe block
    unsafe {
        let v = ((**jre).GetVersion)(jre);
        println!("version: {:?}", v);
    }
}

#[warn(non_snake_case)]
pub extern fn Java_pack_age_Test_helloWorld(jre: *mut JNIEnv, class: *const jclass) {
    println!("Hello World");
}