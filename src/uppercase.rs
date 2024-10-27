pub fn uppercase<'a, const N: usize>(code: &str, buffer: &'a mut [u8; N]) -> Option<&'a str> {
    let iso_candidate = code.as_bytes();
    if iso_candidate.len() == N {
        buffer.copy_from_slice(iso_candidate);
        buffer.make_ascii_uppercase();
        core::str::from_utf8(buffer).ok()
    } else {
        None
    }
}
