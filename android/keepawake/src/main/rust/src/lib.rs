use jni::{objects::JString, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_io_github_segevfiner_keepawake_NativeLib_stringFromJNI<'local>(env: JNIEnv<'local>) -> JString<'local> {
    match env.new_string("Hello from Rust!") {
        Err(_) => JString::default(),
        Ok(s) => s
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
