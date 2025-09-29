#[skipif::skip_if(missing_env(MINIO_TESTING_URL))]
#[cfg(feature = "remote-uploader-s3")]
#[tokio::test]
async fn s3_smoke() {
  use std::path::PathBuf;
  use webview_bundle::remote::uploader::S3Uploader;
  use webview_bundle::{Bundle, BundleReader, Reader};

  let s3 = S3Uploader::builder()
    .endpoint(std::env::var("MINIO_TESTING_URL").unwrap())
    .bucket("webview-bundle")
    .access_key_id("minio_testing")
    .secret_access_key("minio_testing")
    .build()
    .unwrap();
  let mut reader = std::fs::File::open(
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("nextjs.wvb"),
  )
  .unwrap();
  let bundle = Reader::<Bundle>::read(&mut BundleReader::new(&mut reader)).unwrap();
  s3.upload_bundle("nextjs", "1.0.0", &bundle)
    .await
    .expect("fail to upload bundle");
  // TODO: add remote testing
}
