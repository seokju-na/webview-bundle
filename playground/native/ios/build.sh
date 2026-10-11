#!/bin/zsh

xcodebuild -workspace \
  WvbIos.xcworkspace \
  -scheme \
  WvbIos \
  -configuration \
  Debug \
  -destination \
  generic/platform="iOS Simulator" \
  -derivedDataPath \
  out \
  CODE_SIGNING_ALLOWED=NO \
  build
