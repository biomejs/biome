package com.github.biomejs.intellijbiome.widgets

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.BiomeUtils
import com.github.biomejs.intellijbiome.listeners.BIOME_CONFIG_RESOLVED_TOPIC
import com.github.biomejs.intellijbiome.listeners.BiomeConfigResolvedListener
import com.github.biomejs.intellijbiome.lsp.BiomeLspServerSupportProvider
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.application.ModalityState
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.CustomStatusBarWidget
import com.intellij.openapi.wm.StatusBarWidget.WidgetPresentation
import com.intellij.openapi.wm.WindowManager
import com.intellij.openapi.wm.impl.status.EditorBasedWidget
import com.intellij.openapi.wm.impl.status.TextPanel.WithIconAndArrows
import com.intellij.platform.lsp.api.LspServerManager
import com.intellij.platform.lsp.impl.LspServerImpl
import javax.swing.JComponent
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.wm.StatusBarWidget

class BiomeWidget(project: Project) : EditorBasedWidget(project), StatusBarWidget,
    StatusBarWidget.MultipleTextValuesPresentation {
    private val logger: Logger = Logger.getInstance(javaClass)

    init {
        project
            .messageBus
            .connect(this)
            .subscribe(BIOME_CONFIG_RESOLVED_TOPIC, object : BiomeConfigResolvedListener {
                override fun resolved(version: String) {
                    update()
                }
            })
    }

    override fun ID(): String {
        return javaClass.name;
    }

    override fun getPresentation(): WidgetPresentation {
        return this
    }

    override fun getSelectedValue(): String? {
        val biomeBin = BiomeUtils.getBiomeExecutablePath(project);
        val progressManager = ProgressManager.getInstance()

        if (biomeBin == null) {
            return "Biome"
        }

        val version = progressManager.runProcessWithProgressSynchronously<String, Exception>({
            BiomeUtils.getBiomeVersion(project, biomeBin)
        }, BiomeBundle.message("biome.loading"), true, project)


        return "Biome ${version}"
    }

    override fun getTooltipText(): String {
        val lspServerManager = LspServerManager.getInstance(project)
        val lspServer = lspServerManager.getServersForProvider(BiomeLspServerSupportProvider::class.java).firstOrNull()

        return when (lspServer) {
            is LspServerImpl -> {
                if (lspServer.isRunning) {
                    BiomeBundle.message("biome.language.server.is.running")
                } else {
                    BiomeBundle.message("biome.language.server.is.stopped")
                }
            }

            else -> {
                BiomeBundle.message("biome.language.server.is.stopped")
            }
        }
    }

    private fun update() {
        if (myStatusBar == null) {
            logger.warn("Failed to update biome statusbar")
            return
        }

        myStatusBar!!.updateWidget(ID())
    }

}
