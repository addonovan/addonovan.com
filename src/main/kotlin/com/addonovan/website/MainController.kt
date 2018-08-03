@file:Suppress("UNUSED")

package com.addonovan.website

import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.ui.set
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.servlet.HandlerMapping
import javax.servlet.http.HttpServletRequest

@Controller
class MainController {

    // holy fucking shit, why is this variable so damn long?
    // like, this is absolutely ridiculous
    private val PATH_ATTR =
            HandlerMapping.PATH_WITHIN_HANDLER_MAPPING_ATTRIBUTE

    @GetMapping("**")
    fun get(
            request: HttpServletRequest,
            model: Model
    ): String {
        val file = (request.getAttribute(PATH_ATTR) as String).let {
            val file = it.trimStart('/')

            if (file.isEmpty()) {
                "index.html"
            } else {
                "$file.html"
            }
        }

        model["style"] = FileCache["style.css"]
        model["content"] = try {
            FileCache[file]
        }
        catch (e: Exception) {
            FileCache["404.html"]
        }

        return "format"
    }

}

