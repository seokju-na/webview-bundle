import ProjectDescription

let project = Project(
  name: "WvbIos",
  packages: [
    .remote(
      url: "https://github.com/webview-bundle/webview-bundle-ios.git",
      requirement: .exact("0.0.1")
    ),
  ],
  targets: [
    .target(
      name: "WvbIos",
      destinations: .iOS,
      product: .app,
      bundleId: "dev.wvb.playground.ios",
      deploymentTargets: .iOS("16.0"),
      infoPlist: .extendingDefault(with: [
        "UILaunchScreen": .dictionary([:]),
      ]),
      sources: ["Sources/**"],
      resources: [
        .folderReference(path: "bundles"),
        "remote.json",
      ],
      dependencies: [.package(product: "WebViewBundle")],
      settings: .settings(base: ["SWIFT_VERSION": "5.0"])
    ),
  ]
)
