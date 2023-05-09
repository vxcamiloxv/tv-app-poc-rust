package com.androidrust

import android.os.Bundle
import android.view.View
import android.widget.ArrayAdapter
import android.widget.ListView
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity


class MainActivity : AppCompatActivity() {
  var listView: ListView? = null
  var textView: TextView? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    System.loadLibrary("uvn_wasm_rust")
    setContentView(R.layout.activity_main)
    val g = RustGreetings()
    val r = g.sayHello("Rust")
    val images = g.videosImages()
    (findViewById<View>(R.id.greetingField) as TextView).text = r

    listView = findViewById(R.id.listRust);
    textView = findViewById(R.id.textView);
    val adapter: ArrayAdapter<String>? = images?.let { ArrayAdapter<String>(this, R.layout.items_list, it) }
    listView!!.setAdapter(adapter)
  }
}
