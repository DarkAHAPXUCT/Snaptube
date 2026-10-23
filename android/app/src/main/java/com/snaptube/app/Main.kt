package com.snaptube.app
import android.app.Activity
import android.os.Bundle
import android.webkit.WebView
import android.webkit.WebSettings
class Main : Activity() {
    override fun onCreate(settingsBundle: Bundle?) {
        super.onCreate(settingsBundle)
        val browser = WebView(this)
        browser.settings.javaScriptEnabled = true
        browser.settings.cacheMode = WebSettings.LOAD_NO_CACHE
        browser.loadUrl("file:///android_asset/index.html")
        setContentView(browser)
    }
}
