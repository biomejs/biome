package com.github.biomejs.intellijbiome.settings

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.lsp.BiomeLspServerSupportProvider
import com.intellij.javascript.nodejs.library.node_modules.NodeModulesDirectoryManager
import com.intellij.openapi.options.BoundSearchableConfigurable
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogPanel
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServerManager
import com.intellij.ui.dsl.builder.bindText
import com.intellij.ui.dsl.builder.panel

class BiomeSettingsConfigurable(internal val project: Project): BoundSearchableConfigurable(BiomeBundle.message("biome.settings.name"), BiomeBundle.message("biome.settings.name")) {
    override fun createPanel(): DialogPanel {
        val settings: BiomeSettings = BiomeSettings.getInstance(project)

        return panel {
            row {
                textFieldWithBrowseButton(BiomeBundle.message("biome.path.label")) { fileChosen(it) }
                    .bindText(settings::executablePath)
            }

            onApply { LspServerManager.getInstance(project).stopAndRestartIfNeeded(BiomeLspServerSupportProvider::class.java) }
        }

    }

    fun fileChosen(file: VirtualFile): String {
        return file.path
    }
}
