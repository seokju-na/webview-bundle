use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::jni_mangle;
use jni::objects::JObject;

// On Android, initialization must be done before any rustls verification.
// See https://docs.rs/rustls-platform-verifier/latest/rustls_platform_verifier/android/index.html.
#[jni_mangle("dev.wvb.WvbAndroid", "initializeAndroid")]
pub fn initialize_android<'caller>(
  mut env: EnvUnowned<'caller>,
  _this: JObject<'caller>,
  context: JObject<'caller>,
) {
  env
    .with_env(|env| rustls_platform_verifier::android::init_with_env(env, context))
    .resolve::<ThrowRuntimeExAndDefault>();
}
