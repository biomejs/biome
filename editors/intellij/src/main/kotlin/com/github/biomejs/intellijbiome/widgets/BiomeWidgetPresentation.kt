package com.github.biomejs.intellijbiome.widgets

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.BiomeUtils
import com.github.biomejs.intellijbiome.lsp.BiomeLspServerSupportProvider
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.platform.lsp.api.LspServerManager
import com.intellij.platform.lsp.impl.LspServerImpl

@Suppress("UnstableApiUsage")
class BiomeWidgetPresentation(private val project: Project) : StatusBarWidget.MultipleTextValuesPresentation {
    override fun getSelectedValue(): String? {
        val biomeBin = BiomeUtils.getBiomeExecutablePath(project)
        val progressManager = ProgressManager.getInstance()

        if(biomeBin == null) {
            return null
        }

        val version = progressManager.runProcessWithProgressSynchronously<String, Exception>({
            BiomeUtils.getBiomeVersion(project, biomeBin)
        }, BiomeBundle.message("biome.loading"), true, project)


        return "Biome ${version}"
    }

    override fun getTooltipText(): String? {
        val lspServerManager = LspServerManager.getInstance(project)
        val lspServer = lspServerManager.getServersForProvider(BiomeLspServerSupportProvider::class.java).first()

        return when (lspServer) {
            is LspServerImpl -> {
                if (lspServer.isRunning){
                    BiomeBundle.message("biome.language.server.is.running")
                } else {
                    BiomeBundle.message("biome.language.server.is.stopped")
                }
            }

            else -> {
                null
            }
        }

    }
}