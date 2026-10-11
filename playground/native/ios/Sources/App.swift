import SwiftUI
import WebKit
import WebViewBundle

@main
struct WvbIosApp: App {
  var body: some Scene {
    WindowGroup {
      BundleWebView()
    }
  }
}

private struct RemoteConfig: Decodable {
  let baseUrl: String
}

private struct BundleWebView: UIViewRepresentable {
  func makeUIView(context: Context) -> WKWebView {
    do {
      guard let configURL = Bundle.main.url(forResource: "remote", withExtension: "json") else {
        fatalError("Missing remote.json. Run yarn setup first.")
      }
      let remote = try JSONDecoder().decode(RemoteConfig.self, from: Data(contentsOf: configURL))
      let bundle = try WebViewBundle.configure(
        WebViewBundleConfig(
          protocols: [.bundle(scheme: "bundle")],
          updater: WebViewBundleUpdaterConfig(
            remote: WebViewBundleRemoteConfig(baseUrl: remote.baseUrl)
          )
        )
      )
      let webView = bundle.makeWebView()
      #if DEBUG
        if #available(iOS 16.4, *) {
          webView.isInspectable = true
        }
      #endif
      webView.load(URLRequest(url: URL(string: "bundle://testbed.wvb/")!))
      return webView
    } catch {
      fatalError("Failed to initialize WebViewBundle: \(error)")
    }
  }

  func updateUIView(_ uiView: WKWebView, context: Context) {}
}
