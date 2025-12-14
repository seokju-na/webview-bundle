use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::{Buffer, FromNapiValue, Promise, TypeName, ValidateNapiValue};
use napi::{sys, Either, ValueType};
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::signature::{
  EcdsaSecp256r1Signer, EcdsaSecp256r1Verifier, EcdsaSecp384r1Signer, EcdsaSecp384r1Verifier,
  Ed25519Signer, Ed25519Verifier, RsaPkcs1V15Signer, RsaPkcs1V15Verifier, RsaPssSigner,
  RsaPssVerifier, SignatureSigner, SignatureVerifier,
};

#[napi(string_enum = "camelCase", js_name = "SignatureAlgorithm")]
#[derive(PartialEq, Eq)]
pub enum JsSignatureAlgorithm {
  EcdsaSecp256r1,
  EcdsaSecp384r1,
  Ed25519,
  RsaPkcs1V1_5,
  RsaPss,
}

#[napi(string_enum = "camelCase", js_name = "SigningKeyFormat")]
#[derive(PartialEq, Eq)]
pub enum JsSigningKeyFormat {
  Sec1Der,
  Sec1Pem,
  Pkcs1Der,
  Pkcs1Pem,
  Pkcs8Der,
  Pkcs8Pem,
  Raw,
}

#[napi(string_enum = "camelCase", js_name = "VerifyingKeyFormat")]
#[derive(PartialEq, Eq)]
pub enum JsVerifyingKeyFormat {
  SpkiDer,
  SpkiPem,
  Pkcs1Der,
  Pkcs1Pem,
  Sec1,
  Raw,
}

pub struct JsSignatureSigner {
  pub(crate) inner: SignatureSigner,
}

#[napi(object, js_name = "SignatureSignerOptions", object_to_js = false)]
pub struct JsSignatureSignerOptions {
  pub algorithm: JsSignatureAlgorithm,
  pub key: JsSignatureSigningKeyOptions,
}

#[napi(object, js_name = "SignatureSigningKeyOptions", object_to_js = false)]
pub struct JsSignatureSigningKeyOptions {
  pub format: JsSigningKeyFormat,
  #[napi(ts_type = "string | Uint8Array")]
  pub data: Either<String, Buffer>,
}

type NapiSigner = Either<JsSignatureSignerOptions, JsCallback<Buffer, Promise<String>>>;

impl TypeName for JsSignatureSigner {
  fn type_name() -> &'static str {
    NapiSigner::type_name()
  }

  fn value_type() -> ValueType {
    NapiSigner::value_type()
  }
}

impl ValidateNapiValue for JsSignatureSigner {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiSigner::validate(env, napi_val) }
  }
}

impl FromNapiValue for JsSignatureSigner {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiSigner::from_napi_value(env, napi_val)?;
      let unsupported_key_format =
        napi::Error::new(napi::Status::InvalidArg, "unsupported key format");
      let value = match value {
        Either::A(inner) => match &inner.algorithm {
          JsSignatureAlgorithm::EcdsaSecp256r1 => {
            let signer = match &inner.key.format {
              JsSigningKeyFormat::Sec1Der => Ok(
                EcdsaSecp256r1Signer::from_sec1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Sec1Pem => Ok(
                EcdsaSecp256r1Signer::from_sec1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Der => Ok(
                EcdsaSecp256r1Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Pem => Ok(
                EcdsaSecp256r1Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Raw => Ok(
                EcdsaSecp256r1Signer::from_slice(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureSigner::EcdsaSecp256r1(Arc::new(signer))
          }
          JsSignatureAlgorithm::EcdsaSecp384r1 => {
            let signer = match &inner.key.format {
              JsSigningKeyFormat::Sec1Der => Ok(
                EcdsaSecp384r1Signer::from_sec1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Sec1Pem => Ok(
                EcdsaSecp384r1Signer::from_sec1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Der => Ok(
                EcdsaSecp384r1Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Pem => Ok(
                EcdsaSecp384r1Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Raw => Ok(
                EcdsaSecp384r1Signer::from_slice(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureSigner::EcdsaSecp384r1(Arc::new(signer))
          }
          JsSignatureAlgorithm::Ed25519 => {
            let signer = match &inner.key.format {
              JsSigningKeyFormat::Pkcs8Der => Ok(
                Ed25519Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Pem => Ok(
                Ed25519Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Raw => {
                let data = into_buffer_data(inner.key.data)?;
                let bytes = data
                  .get(..64)
                  .and_then(|s| s.try_into().ok())
                  .ok_or_else(|| {
                    napi::Error::new(napi::Status::InvalidArg, "Expect 64 bytes for key pair")
                  })?;
                Ok(
                  Ed25519Signer::from_keypair_bytes(bytes)
                    .map_err(crate::Error::from)
                    .map_err(napi::Error::from)?,
                )
              }
              _ => Err(unsupported_key_format),
            }?;
            SignatureSigner::Ed25519(Arc::new(signer))
          }
          JsSignatureAlgorithm::RsaPkcs1V1_5 => {
            let signer = match &inner.key.format {
              JsSigningKeyFormat::Pkcs1Der => Ok(
                RsaPkcs1V15Signer::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs1Pem => Ok(
                RsaPkcs1V15Signer::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Der => Ok(
                RsaPkcs1V15Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Pem => Ok(
                RsaPkcs1V15Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureSigner::RsaPkcs1V15(Arc::new(signer))
          }
          JsSignatureAlgorithm::RsaPss => {
            let signer = match &inner.key.format {
              JsSigningKeyFormat::Pkcs1Der => Ok(
                RsaPssSigner::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs1Pem => Ok(
                RsaPssSigner::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Der => Ok(
                RsaPssSigner::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsSigningKeyFormat::Pkcs8Pem => Ok(
                RsaPssSigner::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureSigner::RsaPss(Arc::new(signer))
          }
        },
        Either::B(inner) => SignatureSigner::Custom(Arc::new(move |_bundle, message| {
          let message_buf = Buffer::from(message);
          let callback = Arc::clone(&inner);
          Box::pin(async move {
            let ret = callback.invoke_async(message_buf).await?.await?;
            Ok(ret)
          })
        })),
      };
      Ok(Self { inner: value })
    }
  }
}

pub struct JsSignatureVerifier {
  pub(crate) inner: SignatureVerifier,
}

#[napi(object, js_name = "SignatureVerifierOptions", object_to_js = false)]
pub struct JsSignatureVerifierOptions {
  pub algorithm: JsSignatureAlgorithm,
  pub key: JsSignatureVerifyingKeyOptions,
}

#[napi(object, js_name = "SignatureVerifyingKeyOptions", object_to_js = false)]
pub struct JsSignatureVerifyingKeyOptions {
  pub format: JsVerifyingKeyFormat,
  #[napi(ts_type = "string | Uint8Array")]
  pub data: Either<String, Buffer>,
}

type NapiVerifier = Either<JsSignatureVerifierOptions, JsCallback<(Buffer, String), Promise<bool>>>;

impl TypeName for JsSignatureVerifier {
  fn type_name() -> &'static str {
    NapiVerifier::type_name()
  }

  fn value_type() -> ValueType {
    NapiVerifier::value_type()
  }
}

impl ValidateNapiValue for JsSignatureVerifier {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiVerifier::validate(env, napi_val) }
  }
}

impl FromNapiValue for JsSignatureVerifier {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiVerifier::from_napi_value(env, napi_val)?;
      let unsupported_key_format =
        napi::Error::new(napi::Status::InvalidArg, "unsupported key format");
      let value = match value {
        Either::A(inner) => match &inner.algorithm {
          JsSignatureAlgorithm::EcdsaSecp256r1 => {
            let verifier = match &inner.key.format {
              JsVerifyingKeyFormat::Sec1 => Ok(
                EcdsaSecp256r1Verifier::from_sec1_bytes(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiDer => Ok(
                EcdsaSecp256r1Verifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiPem => Ok(
                EcdsaSecp256r1Verifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureVerifier::EcdsaSecp256r1(Arc::new(verifier))
          }
          JsSignatureAlgorithm::EcdsaSecp384r1 => {
            let verifier = match &inner.key.format {
              JsVerifyingKeyFormat::Sec1 => Ok(
                EcdsaSecp384r1Verifier::from_sec1_bytes(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiDer => Ok(
                EcdsaSecp384r1Verifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiPem => Ok(
                EcdsaSecp384r1Verifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureVerifier::EcdsaSecp384r1(Arc::new(verifier))
          }
          JsSignatureAlgorithm::Ed25519 => {
            let verifier = match &inner.key.format {
              JsVerifyingKeyFormat::SpkiDer => Ok(
                Ed25519Verifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiPem => Ok(
                Ed25519Verifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::Raw => {
                let data = into_buffer_data(inner.key.data)?;
                let bytes = data
                  .get(..32)
                  .and_then(|s| s.try_into().ok())
                  .ok_or_else(|| {
                    napi::Error::new(napi::Status::InvalidArg, "Expect 32 bytes for key pair")
                  })?;
                Ok(
                  Ed25519Verifier::from_public_key_bytes(bytes)
                    .map_err(crate::Error::from)
                    .map_err(napi::Error::from)?,
                )
              }
              _ => Err(unsupported_key_format),
            }?;
            SignatureVerifier::Ed25519(Arc::new(verifier))
          }
          JsSignatureAlgorithm::RsaPkcs1V1_5 => {
            let verifier = match &inner.key.format {
              JsVerifyingKeyFormat::Pkcs1Der => Ok(
                RsaPkcs1V15Verifier::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::Pkcs1Pem => Ok(
                RsaPkcs1V15Verifier::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiDer => Ok(
                RsaPkcs1V15Verifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiPem => Ok(
                RsaPkcs1V15Verifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureVerifier::RsaPkcs1V15(Arc::new(verifier))
          }
          JsSignatureAlgorithm::RsaPss => {
            let verifier = match &inner.key.format {
              JsVerifyingKeyFormat::Pkcs1Der => Ok(
                RsaPssVerifier::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::Pkcs1Pem => Ok(
                RsaPssVerifier::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiDer => Ok(
                RsaPssVerifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              JsVerifyingKeyFormat::SpkiPem => Ok(
                RsaPssVerifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            SignatureVerifier::RsaPss(Arc::new(verifier))
          }
        },
        Either::B(inner) => {
          SignatureVerifier::Custom(Arc::new(move |_bundle, message, signature| {
            let message_buf = Buffer::from(message);
            let signature = signature.to_string();
            let callback = Arc::clone(&inner);
            Box::pin(async move {
              let ret = callback
                .invoke_async((message_buf, signature))
                .await?
                .await?;
              Ok(ret)
            })
          }))
        }
      };
      Ok(Self { inner: value })
    }
  }
}

fn into_string_data(d: Either<String, Buffer>) -> napi::Result<String> {
  match d {
    Either::A(s) => Ok(s),
    Either::B(_) => Err(napi::Error::new(
      napi::Status::StringExpected,
      "Expect a string value",
    )),
  }
}

fn into_buffer_data(d: Either<String, Buffer>) -> napi::Result<Buffer> {
  match d {
    Either::A(_) => Err(napi::Error::new(
      napi::Status::ArrayBufferExpected,
      "Expect a array buffer value",
    )),
    Either::B(b) => Ok(b),
  }
}
