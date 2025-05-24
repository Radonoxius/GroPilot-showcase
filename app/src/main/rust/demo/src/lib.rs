use jni::{objects::JClass, sys::jint, JNIEnv};
use jvm_macro::jvm;

static mut I: i32 = 0;

#[jvm("com.gromo.gropilot.NativeApi")]
pub fn increment(
    _env: JNIEnv,
    _this: JClass,
) -> jint {
    unsafe { I += 1 };

    unsafe { I }
}
