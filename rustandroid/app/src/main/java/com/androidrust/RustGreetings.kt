package com.androidrust

import java.util.*
import kotlin.collections.ArrayList

class RustGreetings {

  private external fun greeting(pattern: String): String?
  private external fun fetchVideos(): Array<String>?

  fun sayHello(to: String): String? {
    return greeting(to)
  }

  fun videosImages(): Array<String>? {
    return fetchVideos()
  }
}
