package com.github.biomejs.intellijbiome.lsp

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.BiomeUtils
import com.github.biomejs.intellijbiome.listeners.BIOME_CONFIG_RESOLVED_TOPIC
import com.intellij.execution.ExecutionException
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.diagnostic.thisLogger
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServerSupportProvider
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor
import com.intellij.platform.lsp.api.customization.LspCodeActionsSupport
import com.intellij.platform.lsp.api.customization.LspDiagnosticsSupport
import com.intellij.util.SmartList
import org.eclipse.lsp4j.*

@Suppress("UnstableApiUsage")
class BiomeLspServerSupportProvider : LspServerSupportProvider {

    override fun fileOpened(
        project: Project,
        file: VirtualFile,
        serverStarter: LspServerSupportProvider.LspServerStarter
    ) {
        if (BiomeUtils.isSupportedFileType(file)) {
            val executable = BiomeUtils.getBiomeExecutablePath(project) ?: return
            serverStarter.ensureServerStarted(BiomeLspServerDescriptor(project, executable))
        }
    }
}

@Suppress("UnstableApiUsage")
private class BiomeLspServerDescriptor(project: Project, val executable: String) :
    ProjectWideLspServerDescriptor(project, "Biome") {
    override fun isSupportedFile(file: VirtualFile) = BiomeUtils.isSupportedFileType(file)
    override fun createCommandLine(): GeneralCommandLine {
				val configPath = BiomeUtils.getBiomeConfigPath(project)
				val params = SmartList("lsp-proxy")

				if (!configPath.isNullOrEmpty()) {
					params.add("--config-path")
					params.add(configPath)
				}

        if (executable.isEmpty()) {
            throw ExecutionException(BiomeBundle.message("biome.language.server.not.found"))
        }

        val version = BiomeUtils.getBiomeVersion(project, executable)

        version?.let { project.messageBus.syncPublisher(BIOME_CONFIG_RESOLVED_TOPIC).resolved(it) }

        return BiomeUtils.createNodeCommandLine(project, executable).apply {
            addParameters(params)
        }

    }

    override val lspGoToDefinitionSupport = false
    override val lspCompletionSupport = null
}
