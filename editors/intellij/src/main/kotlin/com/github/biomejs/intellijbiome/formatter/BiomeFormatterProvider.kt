package com.github.biomejs.intellijbiome.formatter

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.BiomeUtils
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.process.CapturingProcessAdapter
import com.intellij.execution.process.OSProcessHandler
import com.intellij.execution.process.ProcessEvent
import com.intellij.formatting.service.AsyncDocumentFormattingService
import com.intellij.formatting.service.AsyncFormattingRequest
import com.intellij.formatting.service.FormattingService.Feature
import com.intellij.openapi.diagnostic.thisLogger
import com.intellij.psi.PsiFile
import com.intellij.util.SmartList
import org.jetbrains.annotations.NotNull
import java.nio.charset.StandardCharsets
import java.util.EnumSet
import com.intellij.execution.ExecutionException

class BiomeFormatterProvider : AsyncDocumentFormattingService() {
    override fun getFeatures(): MutableSet<Feature> = EnumSet.noneOf(Feature::class.java)

    override fun canFormat(file: PsiFile): Boolean =
        file.virtualFile?.let { BiomeUtils.isSupportedFileType(it) } ?: false

    override fun getNotificationGroupId(): String = "Biome"

    override fun getName(): String = "Biome"

    override fun createFormattingTask(request: AsyncFormattingRequest): FormattingTask? {
        val ioFile = request.ioFile ?: return null
        val project = request.context.project
        val params = SmartList("format", "--stdin-file-path", ioFile.path)

        val exePath = BiomeUtils.getBiomeExecutablePath(project)

        if (exePath.isNullOrEmpty()) {
            throw ExecutionException(BiomeBundle.message("biome.language.server.not.found"))
        }

        try {
            val commandLine: GeneralCommandLine = GeneralCommandLine()
                .withExePath(exePath)
                .withParentEnvironmentType(GeneralCommandLine.ParentEnvironmentType.CONSOLE)
                .withInput(ioFile)
                .withParameters(params)

            val handler = OSProcessHandler(commandLine.withCharset(StandardCharsets.UTF_8))
            return object : FormattingTask {
                override fun run() {
                    handler.addProcessListener(object : CapturingProcessAdapter() {
                        override fun processTerminated(@NotNull event: ProcessEvent) {
                            val exitCode = event.exitCode
                            if (exitCode == 0) {
                                request.onTextReady(output.stdout)
                            } else {
                                request.onError(BiomeBundle.message("biome.formatting.failure"), output.stderr)
                            }
                        }
                    })
                    handler.startNotify()
                }

                override fun cancel(): Boolean {
                    handler.destroyProcess()
                    return true
                }

                override fun isRunUnderProgress(): Boolean {
                    return true
                }
            }
        } catch (error: ExecutionException) {
            val message = error.message ?: ""
            request.onError(BiomeBundle.message("biome.formatting.failure"), message)
            return null
        }
    }
}
