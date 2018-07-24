package com.addonovan.website

import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestParam

@Controller
class GreetingController {

    @GetMapping("/greeting")
    fun greeting(
            @RequestParam("name") name: String,
            model: Model
    ) {
        model.addAttribute("name", name)
    }

    @GetMapping("/")
    fun index(model: Model): String {
        model.addAttribute("minecraft", Services.minecraft)
        model.addAttribute("factorio", Services.factorio)
        model.addAttribute("website", Services.website)
        return "index"
    }

}
