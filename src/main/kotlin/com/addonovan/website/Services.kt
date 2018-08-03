package com.addonovan.website

import java.util.concurrent.TimeUnit

private fun serviceRunning(name: String, timeout: Boolean) = Unit.let {
    val proc = ProcessBuilder("./service.sh", name)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

    if (!proc.waitFor(100, TimeUnit.MILLISECONDS)) {
        return timeout
    }

    proc.inputStream.bufferedReader().readText().isNotBlank()
}

object Services {

    private var _minecraft = false

    val minecraft: Boolean
        get() {
            _minecraft = serviceRunning("minecraft", _minecraft)
            return _minecraft
        }

    private var _factorio = false

    val factorio: Boolean
        get() {
            _factorio = serviceRunning("factorio", _factorio)
            return _factorio
        }

}
