package dev.wvb.playground.android

import android.annotation.SuppressLint
import android.app.Activity
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.ViewGroup
import android.view.ViewGroup.LayoutParams.MATCH_PARENT
import android.view.WindowInsets
import android.webkit.WebView
import android.widget.FrameLayout
import dev.wvb.WebViewBundle
import dev.wvb.WebViewBundleConfig
import dev.wvb.WebViewBundleProtocol
import dev.wvb.WebViewBundleRemoteConfig
import dev.wvb.WebViewBundleUpdaterConfig
import org.json.JSONObject

class MainActivity : Activity() {
    private lateinit var webView: WebView
    private var bridge: AutoCloseable? = null

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val remote = assets.open("remote.json").bufferedReader().use { JSONObject(it.readText()) }
        val bundle = WebViewBundle.getInstance(
            this,
            WebViewBundleConfig(
                protocols = listOf(WebViewBundleProtocol.bundle()),
                updater = WebViewBundleUpdaterConfig(
                    remote = WebViewBundleRemoteConfig(baseUrl = remote.getString("baseUrl")),
                ),
                onError = { error -> Log.e("WvbPlayground", "Bundle request failed", error) },
            ),
        )

        WebView.setWebContentsDebuggingEnabled(BuildConfig.DEBUG)
        webView = WebView(this).apply {
            settings.javaScriptEnabled = true
            settings.domStorageEnabled = true
            settings.allowFileAccess = false
            settings.allowContentAccess = false
        }
        bridge = bundle.install(webView)

        val root = FrameLayout(this).apply {
            addView(webView, FrameLayout.LayoutParams(MATCH_PARENT, MATCH_PARENT))
        }
        setContentView(root)

        // Android 15 enforces edge-to-edge; keep the testbed clear of system bars and cutouts.
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.VANILLA_ICE_CREAM) {
            root.setOnApplyWindowInsetsListener { view, insets ->
                val safe = insets.getInsets(
                    WindowInsets.Type.systemBars() or WindowInsets.Type.displayCutout(),
                )
                view.setPadding(safe.left, safe.top, safe.right, safe.bottom)
                WindowInsets.CONSUMED
            }
            root.requestApplyInsets()
        }

        webView.loadUrl("https://testbed.wvb/")
    }

    override fun onDestroy() {
        bridge?.close()
        if (::webView.isInitialized) {
            (webView.parent as? ViewGroup)?.removeView(webView)
            webView.destroy()
        }
        super.onDestroy()
    }
}
