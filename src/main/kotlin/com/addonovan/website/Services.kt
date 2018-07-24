package com.addonovan.website

import jdk.nashorn.internal.objects.NativeRegExp.exec
import java.io.InputStream
import java.util.concurrent.TimeUnit

private fun serviceRunning(name: String) = Unit.let {
    val proc = ProcessBuilder(*arrayOf("./service.sh", name))
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

    if (!proc.waitFor(25, TimeUnit.MILLISECONDS)) {
        throw RuntimeException("Process timeout")
    }

    proc.inputStream.bufferedReader().readText().isNotBlank()
}

object Services {

    val minecraft = serviceRunning("minecraft")

    val factorio = serviceRunning("factorio")

    val website = serviceRunning("website")

}
