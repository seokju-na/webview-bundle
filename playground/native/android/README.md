# Android playground

Uses `dev.wvb:webview-bundle-android:0.0.3` from Maven Central. Only `google()` and
`mavenCentral()` are needed for dependencies; no additional rustls repository is required.

Prerequisites: JDK 17 or newer (JDK 21 recommended), Android SDK Platform 35, and Build Tools 35.0.0.
Set `JAVA_HOME` to your JDK and `ANDROID_HOME` to your SDK, or open this directory in Android Studio
and configure its Gradle JDK and SDK location. The Gradle wrapper is included.

```sh
cd playground/native/android
yarn setup
yarn builtin
yarn build

# With an emulator or device connected:
yarn install:app
adb shell am start -n dev.wvb.playground.android/.MainActivity
```
