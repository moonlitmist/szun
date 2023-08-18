pub fn name_indexof(key:&str) -> usize
{
    unsafe {super::name_indexof(key.as_ptr(), key.len())}
}

pub fn name_keyof(index:usize) -> String
{
    let str = unsafe {super::name_keyof(index)};
    let bytes = Vec::<u8>::from(unsafe {std::slice::from_raw_parts(str.bytes, str.length)});
    let result = match String::from_utf8(bytes) {
        Ok(str) => str,
        Err(_) => String::new()
    };
    unsafe { super::name_release(str); }
    return result;
}
