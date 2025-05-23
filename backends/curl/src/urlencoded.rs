use std::ffi::c_char;

use curl::easy::Easy;
use curl_sys::curl_free;
use memchr::memmem;

pub fn curl_escape(easy: &Easy, str: impl AsRef<[u8]>) -> Vec<u8> {
    struct CurlString(*mut c_char);
    impl Drop for CurlString {
        fn drop(&mut self) {
            unsafe {
                curl_free(self.0 as _);
            }
        }
    }
    let str = str.as_ref();
    if str.is_empty() {
        return vec![];
    }
    let mut buf = unsafe {
        let raw_str = CurlString(curl_sys::curl_easy_escape(
            easy.raw(),
            str.as_ptr() as _,
            str.len().try_into().expect("str escaped too long"),
        ));
        std::ffi::CStr::from_ptr(raw_str.0).to_bytes().to_vec()
    };
    {
        // replace %20 with +
        let mut idx = 0;
        while let Some(pos) = memmem::find(&buf[idx..], b"%20") {
            buf.splice(idx + pos..idx + pos + 3, *b"+");
            idx += pos + 1;
        }
    }

    buf
}
