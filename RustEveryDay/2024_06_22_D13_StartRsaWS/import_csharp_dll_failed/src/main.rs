#[link(name = "rsa1024", kind = "static")]
extern "C" {
    fn Rsa1024_GeneratePrivateKeyXml() -> *const std::os::raw::c_char;
}


fn main() {
    println!("Hello, world!");


    unsafe {
        let private_key_xml = std::ffi::CStr::from_ptr(Rsa1024_GeneratePrivateKeyXml());
        let private_key_xml_str = private_key_xml.to_str().expect("Invalid UTF-8 string");
        println!("Private Key XML: {}", private_key_xml_str);
    }
}
