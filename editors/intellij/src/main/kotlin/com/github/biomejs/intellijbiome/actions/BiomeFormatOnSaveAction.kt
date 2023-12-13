package com.github.biomejs.intellijbiome.actions

import com.github.biomejs.intellijbiome.settings.BiomeSettings
import com.intellij.codeInsight.actions.onSave.FormatOnSaveOptions
import com.intellij.ide.actionsOnSave.impl.ActionsOnSaveFileDocumentManagerListener
import com.intellij.lang.javascript.linter.GlobPatternUtil
import com.intellij.openapi.editor.Document
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.util.containers.ContainerUtil


class BiomeFormatOnSaveAction : ActionsOnSaveFileDocumentManagerListener.ActionOnSave() {
	override fun isEnabledForProject(project: Project): Boolean {
		val settings = BiomeSettings.getInstance(project)
		return settings.formatOnSave
	}

	override fun processDocuments(project: Project, documents: Array<Document?>) {
		val settings = BiomeSettings.getInstance(project)

		if (!settings.formatOnSave) {
			return
		}

		val manager = FileDocumentManager.getInstance()

		val files: List<VirtualFile> = ContainerUtil.mapNotNull(documents) { document ->
			val file = document?.let { manager.getFile(it) }
			file
		}

		val matchingFiles =
			GlobPatternUtil.filterFilesMatchingGlobPattern(project, settings.formatFilePattern, files)

		if (matchingFiles.isNotEmpty()) {
//			ReformatWithPrettierAction.processVirtualFiles(project, matchingFiles, NOOP_ERROR_HANDLER)
		}
	}
}

//
//internal class PrettierActionOnSave : ActionsOnSaveFileDocumentManagerListener.ActionOnSave() {
//	fun isEnabledForProject(@NotNull project: Project?): Boolean {
//		val configuration: Unit = PrettierConfiguration.getInstance(project)
//		return configuration.isRunOnSave()
//	}
//
//	fun processDocuments(@NotNull project: Project?, @NotNull documents: @NotNull Array<Document?>?) {
//		val prettierConfiguration: PrettierConfiguration = PrettierConfiguration.getInstance(project)
//		if (!prettierConfiguration.isRunOnSave()) {
//			return
//		}
//
//		val manager = FileDocumentManager.getInstance()
//		val files: List<VirtualFile> = ContainerUtil.mapNotNull(documents) { document ->
//			val file = manager.getFile(document)
//			if (file != null && prettierConfiguration.isRunOnReformat()) {
//				val onSaveOptions = FormatOnSaveOptions.getInstance(project)
//				if (onSaveOptions.isRunOnSaveEnabled &&
//					(onSaveOptions.isAllFileTypesSelected || onSaveOptions.isFileTypeSelected(file.fileType))
//				) {
//					// already processed as com.intellij.prettierjs.PrettierPostFormatProcessor
//					return@mapNotNull null
//				}
//			}
//			file
//		}
//
//		val matchingFiles =
//			GlobPatternUtil.filterFilesMatchingGlobPattern(project, prettierConfiguration.getFilesPattern(), files)
//
//		if (!matchingFiles.isEmpty()) {
//			ReformatWithPrettierAction.processVirtualFiles(project, matchingFiles, NOOP_ERROR_HANDLER)
//		}
//	}
//
//	companion object {
//		val NOOP_ERROR_HANDLER: ReformatWithPrettierAction.ErrorHandler = object : ErrorHandler() {
//			fun showError(
//				@NotNull project: Project?,
//				@Nullable editor: Editor?,
//				@NotNull text: String?,
//				@Nullable onLinkClick: Runnable?
//			) {
//				// No need to show any notification in case of 'Prettier on save' failure. Most likely the file is simply not syntactically valid at the moment.
//			}
//		}
//	}
//}
