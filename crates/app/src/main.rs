fn main() {
    let from_c = unsafe { clibs_sys::c_lib_add_together(1, 1) };
    println!("from_c: {from_c}");

    let from_cpp = unsafe { clibs_sys::cpp_lib_add(2, 2) };
    println!("from_cpp: {from_cpp}");

    unsafe {
        clibs_sys::cpp_lib_add_throws(1, 1);
    };
}
