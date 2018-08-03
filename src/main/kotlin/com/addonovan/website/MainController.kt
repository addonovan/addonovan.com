@file:Suppress("UNUSED")

package com.addonovan.website

import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.ui.set
import org.springframework.web.bind.annotation.GetMapping

@Controller
class MainController {

    @GetMapping("")
    fun index(model: Model): String {
        model["style"] = FileCache["style.css"]
        model["content"] = FileCache["index.html"]
        return "format"
    }

}

