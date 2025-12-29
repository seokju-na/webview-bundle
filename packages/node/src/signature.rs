use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::{Buffer, FnArgs, FromNapiValue, Promise, TypeName, ValidateNapiValue};
use napi::{sys, Either, ValueType};
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::signature;

#[napi(string_enum = "camelCase")]
#[derive(PartialEq, Eq)]
pub enum SignatureAlgorithm {
  EcdsaSecp256r1,
  EcdsaSecp384r1,
  Ed25519,
  RsaPkcs1V1_5,
  RsaPss,
}

#[napi(string_enum = "camelCase")]
#[derive(PartialEq, Eq)]
pub enum SigningKeyFormat {
  Sec1Der,
  Sec1Pem,
  Pkcs1Der,
  Pkcs1Pem,
  Pkcs8Der,
  Pkcs8Pem,
  Raw,
}

#[napi(string_enum = "camelCase")]
#[derive(PartialEq, Eq)]
pub enum VerifyingKeyFormat {
  SpkiDer,
  SpkiPem,
  Pkcs1Der,
  Pkcs1Pem,
  Sec1,
  Raw,
}

pub struct SignatureSigner {
  pub(crate) inner: signature::SignatureSigner,
}

#[napi(object, object_to_js = false)]
pub struct SignatureSignerOptions {
  pub algorithm: SignatureAlgorithm,
  pub key: SignatureSigningKeyOptions,
}

#[napi(object, object_to_js = false)]
pub struct SignatureSigningKeyOptions {
  pub format: SigningKeyFormat,
  #[napi(ts_type = "string | Uint8Array")]
  pub data: Either<String, Buffer>,
}

type NapiSigner = Either<SignatureSignerOptions, JsCallback<Buffer, Promise<String>>>;

impl TypeName for SignatureSigner {
  fn type_name() -> &'static str {
    NapiSigner::type_name()
  }

  fn value_type() -> ValueType {
    NapiSigner::value_type()
  }
}

impl ValidateNapiValue for SignatureSigner {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiSigner::validate(env, napi_val) }
  }
}

impl FromNapiValue for SignatureSigner {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiSigner::from_napi_value(env, napi_val)?;
      let unsupported_key_format =
        napi::Error::new(napi::Status::InvalidArg, "unsupported key format");
      let value = match value {
        Either::A(inner) => match &inner.algorithm {
          SignatureAlgorithm::EcdsaSecp256r1 => {
            let signer = match &inner.key.format {
              SigningKeyFormat::Sec1Der => Ok(
                signature::EcdsaSecp256r1Signer::from_sec1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Sec1Pem => Ok(
                signature::EcdsaSecp256r1Signer::from_sec1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Der => Ok(
                signature::EcdsaSecp256r1Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Pem => Ok(
                signature::EcdsaSecp256r1Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Raw => Ok(
                signature::EcdsaSecp256r1Signer::from_slice(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureSigner::EcdsaSecp256r1(Arc::new(signer))
          }
          SignatureAlgorithm::EcdsaSecp384r1 => {
            let signer = match &inner.key.format {
              SigningKeyFormat::Sec1Der => Ok(
                signature::EcdsaSecp384r1Signer::from_sec1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Sec1Pem => Ok(
                signature::EcdsaSecp384r1Signer::from_sec1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Der => Ok(
                signature::EcdsaSecp384r1Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Pem => Ok(
                signature::EcdsaSecp384r1Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Raw => Ok(
                signature::EcdsaSecp384r1Signer::from_slice(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureSigner::EcdsaSecp384r1(Arc::new(signer))
          }
          SignatureAlgorithm::Ed25519 => {
            let signer = match &inner.key.format {
              SigningKeyFormat::Pkcs8Der => Ok(
                signature::Ed25519Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Pem => Ok(
                signature::Ed25519Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Raw => {
                let data = into_buffer_data(inner.key.data)?;
                let bytes = data
                  .get(..64)
                  .and_then(|s| s.try_into().ok())
                  .ok_or_else(|| {
                    napi::Error::new(napi::Status::InvalidArg, "Expect 64 bytes for key pair")
                  })?;
                Ok(
                  signature::Ed25519Signer::from_keypair_bytes(bytes)
                    .map_err(crate::Error::from)
                    .map_err(napi::Error::from)?,
                )
              }
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureSigner::Ed25519(Arc::new(signer))
          }
          SignatureAlgorithm::RsaPkcs1V1_5 => {
            let signer = match &inner.key.format {
              SigningKeyFormat::Pkcs1Der => Ok(
                signature::RsaPkcs1V15Signer::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs1Pem => Ok(
                signature::RsaPkcs1V15Signer::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Der => Ok(
                signature::RsaPkcs1V15Signer::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Pem => Ok(
                signature::RsaPkcs1V15Signer::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureSigner::RsaPkcs1V15(Arc::new(signer))
          }
          SignatureAlgorithm::RsaPss => {
            let signer = match &inner.key.format {
              SigningKeyFormat::Pkcs1Der => Ok(
                signature::RsaPssSigner::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs1Pem => Ok(
                signature::RsaPssSigner::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Der => Ok(
                signature::RsaPssSigner::from_pkcs8_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              SigningKeyFormat::Pkcs8Pem => Ok(
                signature::RsaPssSigner::from_pkcs8_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureSigner::RsaPss(Arc::new(signer))
          }
        },
        Either::B(inner) => {
          signature::SignatureSigner::Custom(Arc::new(move |_bundle, message| {
            let message_buf = Buffer::from(message);
            let callback = Arc::clone(&inner);
            Box::pin(async move {
              let ret = callback.invoke_async(message_buf).await?.await?;
              Ok(ret)
            })
          }))
        }
      };
      Ok(Self { inner: value })
    }
  }
}

pub struct SignatureVerifier {
  pub(crate) inner: signature::SignatureVerifier,
}

#[napi(object, object_to_js = false)]
pub struct SignatureVerifierOptions {
  pub algorithm: SignatureAlgorithm,
  pub key: SignatureVerifyingKeyOptions,
}

#[napi(object, object_to_js = false)]
pub struct SignatureVerifyingKeyOptions {
  pub format: VerifyingKeyFormat,
  #[napi(ts_type = "string | Uint8Array")]
  pub data: Either<String, Buffer>,
}

type NapiVerifier =
  Either<SignatureVerifierOptions, JsCallback<FnArgs<(Buffer, String)>, Promise<bool>>>;

impl TypeName for SignatureVerifier {
  fn type_name() -> &'static str {
    NapiVerifier::type_name()
  }

  fn value_type() -> ValueType {
    NapiVerifier::value_type()
  }
}

impl ValidateNapiValue for SignatureVerifier {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiVerifier::validate(env, napi_val) }
  }
}

impl FromNapiValue for SignatureVerifier {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiVerifier::from_napi_value(env, napi_val)?;
      let unsupported_key_format =
        napi::Error::new(napi::Status::InvalidArg, "unsupported key format");
      let value = match value {
        Either::A(inner) => match &inner.algorithm {
          SignatureAlgorithm::EcdsaSecp256r1 => {
            let verifier = match &inner.key.format {
              VerifyingKeyFormat::Sec1 => Ok(
                signature::EcdsaSecp256r1Verifier::from_sec1_bytes(&into_buffer_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiDer => Ok(
                signature::EcdsaSecp256r1Verifier::from_public_key_der(&into_buffer_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiPem => Ok(
                signature::EcdsaSecp256r1Verifier::from_public_key_pem(&into_string_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureVerifier::EcdsaSecp256r1(Arc::new(verifier))
          }
          SignatureAlgorithm::EcdsaSecp384r1 => {
            let verifier = match &inner.key.format {
              VerifyingKeyFormat::Sec1 => Ok(
                signature::EcdsaSecp384r1Verifier::from_sec1_bytes(&into_buffer_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiDer => Ok(
                signature::EcdsaSecp384r1Verifier::from_public_key_der(&into_buffer_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiPem => Ok(
                signature::EcdsaSecp384r1Verifier::from_public_key_pem(&into_string_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureVerifier::EcdsaSecp384r1(Arc::new(verifier))
          }
          SignatureAlgorithm::Ed25519 => {
            let verifier = match &inner.key.format {
              VerifyingKeyFormat::SpkiDer => Ok(
                signature::Ed25519Verifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiPem => Ok(
                signature::Ed25519Verifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::Raw => {
                let data = into_buffer_data(inner.key.data)?;
                let bytes = data
                  .get(..32)
                  .and_then(|s| s.try_into().ok())
                  .ok_or_else(|| {
                    napi::Error::new(napi::Status::InvalidArg, "Expect 32 bytes for key pair")
                  })?;
                Ok(
                  signature::Ed25519Verifier::from_public_key_bytes(bytes)
                    .map_err(crate::Error::from)
                    .map_err(napi::Error::from)?,
                )
              }
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureVerifier::Ed25519(Arc::new(verifier))
          }
          SignatureAlgorithm::RsaPkcs1V1_5 => {
            let verifier = match &inner.key.format {
              VerifyingKeyFormat::Pkcs1Der => Ok(
                signature::RsaPkcs1V15Verifier::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::Pkcs1Pem => Ok(
                signature::RsaPkcs1V15Verifier::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiDer => Ok(
                signature::RsaPkcs1V15Verifier::from_public_key_der(&into_buffer_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiPem => Ok(
                signature::RsaPkcs1V15Verifier::from_public_key_pem(&into_string_data(
                  inner.key.data,
                )?)
                .map_err(crate::Error::from)
                .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureVerifier::RsaPkcs1V15(Arc::new(verifier))
          }
          SignatureAlgorithm::RsaPss => {
            let verifier = match &inner.key.format {
              VerifyingKeyFormat::Pkcs1Der => Ok(
                signature::RsaPssVerifier::from_pkcs1_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::Pkcs1Pem => Ok(
                signature::RsaPssVerifier::from_pkcs1_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiDer => Ok(
                signature::RsaPssVerifier::from_public_key_der(&into_buffer_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              VerifyingKeyFormat::SpkiPem => Ok(
                signature::RsaPssVerifier::from_public_key_pem(&into_string_data(inner.key.data)?)
                  .map_err(crate::Error::from)
                  .map_err(napi::Error::from)?,
              ),
              _ => Err(unsupported_key_format),
            }?;
            signature::SignatureVerifier::RsaPss(Arc::new(verifier))
          }
        },
        Either::B(inner) => {
          signature::SignatureVerifier::Custom(Arc::new(move |_bundle, message, signature| {
            let message_buf = Buffer::from(message);
            let signature = signature.to_string();
            let callback = Arc::clone(&inner);
            Box::pin(async move {
              let ret = callback
                .invoke_async((message_buf, signature).into())
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
