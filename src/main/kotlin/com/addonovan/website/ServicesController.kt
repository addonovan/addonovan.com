@file:Suppress("UNUSED")

package com.addonovan.website

import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping

@Controller
@RequestMapping("services")
class ServicesController {

    @GetMapping("")
    fun index(model: Model): String {
        // get the status of all of the services
        status(model)

        return "services/index"
    }

    @GetMapping("status")
    fun status(model: Model) {
        model.addAttribute("minecraft", Services.minecraft)
        model.addAttribute("factorio", Services.factorio)
        model.addAttribute("website", Services.website)
    }

}
