package com.github.biomejs.intellijbiome.settings

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.services.BiomeServerService
import com.intellij.openapi.components.service
import com.intellij.openapi.options.BoundSearchableConfigurable
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogPanel
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.ui.dsl.builder.bindText
import com.intellij.ui.dsl.builder.panel

class BiomeSettingsConfigurable(internal val project: Project) :
    BoundSearchableConfigurable(
        BiomeBundle.message("biome.settings.name"),
        BiomeBundle.message("biome.settings.name")
    ) {
    override fun createPanel(): DialogPanel {
        val settings: BiomeSettings = BiomeSettings.getInstance(project)
        val biomeServerService = project.service<BiomeServerService>()

        return panel {
            row(BiomeBundle.message("biome.path.label")) {
                textFieldWithBrowseButton(BiomeBundle.message("biome.path.label")) { fileChosen(it) }
                    .bindText(settings::executablePath)
            }

            onApply {
                biomeServerService.restartBiomeServer()
                biomeServerService.notifyRestart()
            }
        }

    }

    fun fileChosen(file: VirtualFile): String {
        return file.path
    }
}
