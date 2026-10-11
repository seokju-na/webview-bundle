package dev.wvb

import android.content.Context
import androidx.annotation.Keep

/** Android runtime required by the native HTTPS certificate verifier. */
@Keep
object WvbAndroid {
    init {
        System.loadLibrary("wvb_ffi")
    }

    /**
     * Call before using Remote or ProxyProtocolHandler directly. Repeated calls are safe.
     * Only the application context is retained for the lifetime of the process.
     */
    fun initialize(context: Context) {
        initializeAndroid(context.applicationContext)
    }

    private external fun initializeAndroid(context: Context)
}
