package com.github.biomejs.intellijbiome

import com.github.biomejs.intellijbiome.settings.BiomeSettings
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.util.ExecUtil
import com.intellij.javascript.nodejs.library.node_modules.NodeModulesDirectoryManager
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.util.SmartList
import java.io.File
import com.intellij.javascript.nodejs.interpreter.NodeCommandLineConfigurator
import com.intellij.javascript.nodejs.interpreter.NodeJsInterpreterManager
import com.intellij.javascript.nodejs.interpreter.local.NodeJsLocalInterpreter
import com.intellij.javascript.nodejs.interpreter.wsl.WslNodeInterpreter
import com.intellij.execution.ExecutionException

object BiomeUtils {
    fun isSupportedFileType(file: VirtualFile): Boolean = when (file.extension) {
        "js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx", "d.ts", "json", "jsonc" -> true
        else -> false
    }

    fun getBiomeVersion(project: Project, binaryPath: String): String? {
        if (binaryPath.isEmpty()) {
            return null
        }

        val versionRegex = Regex("\\d{1,2}\\.\\d{1,2}\\.\\d{1,3}")

        val commandLine = createNodeCommandLine(project, binaryPath).apply {
            addParameter("--version")
        }

        val output = ExecUtil.execAndGetOutput(commandLine)

        val matchResult = versionRegex.find(output.stdout)

        return matchResult?.value
    }

    fun getBiomeExecutablePath(project: Project): String? {
        val directoryManager = NodeModulesDirectoryManager.getInstance(project)
        val executablePath = BiomeSettings.getInstance(project).executablePath
        val biomeBinFile = directoryManager.nodeModulesDirs
            .asSequence()
            .mapNotNull { it.findFileByRelativePath(".bin/biome") }
            .filter { it.isValid }
            .firstOrNull()

        if (executablePath.isEmpty()) {
            return biomeBinFile?.path
        }

        return executablePath
    }

    fun createNodeCommandLine(project: Project, executable: String): GeneralCommandLine {
        val interpreter = NodeJsInterpreterManager.getInstance(project).interpreter
        if (interpreter !is NodeJsLocalInterpreter && interpreter !is WslNodeInterpreter) {
            throw ExecutionException(BiomeBundle.message("biome.interpreter.not.configured"))
        }

        return GeneralCommandLine().apply {
            withParentEnvironmentType(GeneralCommandLine.ParentEnvironmentType.CONSOLE)
            addParameter(executable)

            NodeCommandLineConfigurator.find(interpreter)
                .configure(this, NodeCommandLineConfigurator.defaultOptions(project))
        }
    }
}
