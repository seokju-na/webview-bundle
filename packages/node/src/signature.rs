use napi_derive::napi;
use webview_bundle::signature::SignatureSigner;

#[napi(string_enum = "camelCase")]
pub enum JsSignatureAlgorithm {
  EcdsaSecp256r1,
  EcdsaSecp384r1,
  Ed25519,
  RsaPkcs1V1_5,
  RsaPss,
}

#[napi(string_enum = "camelCase")]
pub enum JsSigningKeyFormat {
  Sec1Der,
  Sec1Pem,
  Pkcs1Der,
  Pkcs1Pem,
  Pkcs8Der,
  Pkcs8Pem,
  Raw,
}

#[napi(string_enum = "camelCase")]
pub enum JsVerifyingKeyFormat {
  SpkiDer,
  SpkiPem,
  Pkcs1Der,
  Pkcs1Pem,
  Raw,
}

pub struct JsSignatureSigner {
  pub(crate) inner: SignatureSigner,
}
