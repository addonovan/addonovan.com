package com.addonovan.website

import java.io.File
import java.io.FileNotFoundException
import java.util.*

private data class FileInfo(
        val modified: Long,
        val content: String
)

object FileCache {

    private val cache = WeakHashMap<String, FileInfo>()

    operator fun get(fileName: String): String {
        val file = File("src/main/resources/content/$fileName")

        if (!file.exists()) {
            val message = "No file at path: ${file.absolutePath}"
            throw FileNotFoundException(message)
        }

        val lastModified = file.lastModified()

        var info = cache[fileName]?.let {
            if (it.modified < lastModified)
                null
            else
                it
        }

        // if we need to re-read the contents, for whatever reason,
        // then we'll do that now
        if (info == null) {
            info = FileInfo(lastModified, file.readText())
            cache[fileName] = info
        }

        return info.content
    }

}
