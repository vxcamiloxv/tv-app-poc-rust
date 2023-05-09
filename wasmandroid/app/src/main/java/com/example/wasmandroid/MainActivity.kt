package com.example.wasmandroid

import android.os.Build
import android.os.Bundle
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity
import java.io.ByteArrayInputStream
import java.io.IOException


/**
 * Loads wasm application.
 */
class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val webView = findViewById<View>(R.id.webview) as WebView
        val settings = webView.settings
        settings.javaScriptEnabled = true
        webView.webViewClient = object : WebViewClient() {
            override fun shouldInterceptRequest(
                view: WebView,
                request: WebResourceRequest
            ): WebResourceResponse? {
                val path = request.url.path!!.replace("/assets/", "")
                var ext =  path.substringAfterLast('.', "")
                return try {
                    val mime: String
                    val assetManager = assets

                    when (ext) {
                        "html" -> mime = "text/html"
                        "wasm" -> mime = "application/wasm"
                        "js" -> mime = "text/javascript"
                        "css" -> mime = "text/css"
                        else -> {
                            return super.shouldInterceptRequest(view, request)
                        }
                    }

                    var input = assetManager.open(path)
                    WebResourceResponse(mime, "utf-8", input)
                } catch (e: IOException) {
                    e.printStackTrace()
                    val result = ByteArrayInputStream("X:$path E:$e".toByteArray())
                    WebResourceResponse("text/plain", "utf-8", result)
                }
            }
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
            WebView.setWebContentsDebuggingEnabled(true)
        }

        webView.loadUrl("https://appassets.androidplatform.net/assets/www/index.html")
    }
}
