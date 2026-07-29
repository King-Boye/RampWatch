#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct JwtInput {
    // Generate arbitrary JWT-like structures with base64url components
    #[arbitrary(with = "arbitrary_base64url")]
    header: String,
    #[arbitrary(with = "arbitrary_base64url")]
    payload: String,
    #[arbitrary(with = "arbitrary_base64url")]
    signature: String,
}

fn arbitrary_base64url(u: &mut arbitrary::Unstructured) -> arbitrary::Result<String> {
    // Generate valid base64url characters
    const BASE64URL_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let len = u.int_in_range(0..=128)?;
    let mut result = String::new();
    for _ in 0..len {
        let idx = u.int_in_range(0..BASE64URL_CHARS.len())?;
        result.push(BASE64URL_CHARS[idx] as char);
    }
    Ok(result)
}

fuzz_target!(|input: JwtInput| {
    // Construct a JWT-like structure: header.payload.signature
    let jwt = format!("{}.{}.{}", input.header, input.payload, input.signature);

    // Test base64url decoding of individual components
    let _ = anchorkit::sep10_jwt::base64url_decode(input.header.as_bytes());
    let _ = anchorkit::sep10_jwt::base64url_decode(input.payload.as_bytes());
    let _ = anchorkit::sep10_jwt::base64url_decode(input.signature.as_bytes());

    // Ensure no panics occur during decoding operations
    // Real JWT verification would be tested, but this focuses on panic-freedom
    // for the low-level base64url decoder
});
