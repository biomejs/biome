package com.github.biomejs.intellijbiome.listeners

import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.newvfs.BulkFileListener
import com.intellij.openapi.vfs.newvfs.events.VFileEvent
import com.intellij.openapi.components.service
import com.github.biomejs.intellijbiome.services.BiomeServerService

class BiomeConfigListener(val project: Project) : BulkFileListener {
    override fun after(events: MutableList<out VFileEvent>) {
        super.after(events)
        events.forEach {
            if (it.file?.name?.contains("biome.json") == true) {
                val biomeServerService = project.service<BiomeServerService>()

                biomeServerService.restartBiomeServer()
            }
        }
    }
}
