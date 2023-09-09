package com.github.biomejs.intellijbiome.lsp

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.BiomeUtils
import com.intellij.execution.ExecutionException
import com.intellij.execution.configurations.GeneralCommandLine
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
    override fun fileOpened(project: Project, file: VirtualFile, serverStarter: LspServerSupportProvider.LspServerStarter) {
        if (BiomeUtils.isSupportedFileType(file.fileType)) {
            val executable = BiomeUtils.getBiomeExecutablePath(project) ?: return

            serverStarter.ensureServerStarted(BiomeLspServerDescriptor(project, executable))
        }
    }
}

@Suppress("UnstableApiUsage")
private class BiomeLspServerDescriptor(project: Project, val executable: String) : ProjectWideLspServerDescriptor(project, "Biome") {
    override fun isSupportedFile(file: VirtualFile) = BiomeUtils.isSupportedFileType(file.fileType)
    override fun createCommandLine(): GeneralCommandLine {
        val params = SmartList("lsp-proxy")
        BiomeUtils.attachConfigPath(params, project, thisLogger())

        if(executable.isEmpty()) {
            throw ExecutionException(BiomeBundle.message("biome.language.server.not.found"))
        }

        return GeneralCommandLine()
            .withExePath(executable)
            .withParameters(params)
    }

    override val lspGoToDefinitionSupport = false
    override val lspCompletionSupport = null
    override val lspDiagnosticsSupport: LspDiagnosticsSupport
        get() = BiomeLspDiagnosticsSupport()
    override val lspCodeActionsSupport: LspCodeActionsSupport
        get() = BiomeLspCodeActionsSupport()

    override val clientCapabilities: ClientCapabilities
        get() = super.clientCapabilities.apply {
            textDocument = TextDocumentClientCapabilities().apply {
                codeAction = CodeActionCapabilities().apply {
                    codeActionLiteralSupport = CodeActionLiteralSupportCapabilities().apply {
                        codeActionKind = CodeActionKindCapabilities(listOf(CodeActionKind.Empty))
                    }
                }
            }
        }
}
