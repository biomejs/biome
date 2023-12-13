package com.github.biomejs.intellijbiome.settings

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.services.BiomeServerService
import com.intellij.ide.actionsOnSave.ActionsOnSaveConfigurable
import com.intellij.lang.javascript.JavaScriptBundle
import com.intellij.openapi.Disposable
import com.intellij.openapi.application.ApplicationNamesInfo
import com.intellij.openapi.components.service
import com.intellij.openapi.observable.properties.ObservableMutableProperty
import com.intellij.openapi.observable.util.whenItemSelected
import com.intellij.openapi.options.BoundSearchableConfigurable
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogPanel
import com.intellij.openapi.ui.ValidationInfo
import com.intellij.openapi.util.NlsSafe
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.ui.ContextHelpLabel
import com.intellij.ui.components.JBRadioButton
import com.intellij.ui.components.JBTextField
import com.intellij.ui.dsl.builder.*
import com.intellij.ui.layout.not
import com.intellij.ui.layout.selected
import com.intellij.util.ui.JBUI
import com.intellij.util.ui.UIUtil
import java.nio.file.FileSystems
import java.util.regex.PatternSyntaxException
import javax.swing.JCheckBox
import javax.swing.JRadioButton
import javax.swing.text.JTextComponent

class BiomeConfigurable(internal val project: Project) :
	BoundSearchableConfigurable(
		BiomeBundle.message("biome.settings.name"),
		BiomeBundle.message("biome.settings.name")
	) {
	private lateinit var runFormatForFilesField: JBTextField
	private lateinit var runLintForFilesField: JBTextField
	public lateinit var runOnSaveCheckBox: JCheckBox

	private lateinit var disabledConfiguration: JRadioButton
	private lateinit var automaticConfiguration: JRadioButton
	private lateinit var manualConfiguration: JRadioButton
	override fun createPanel(): DialogPanel {
		val settings: BiomeSettings = BiomeSettings.getInstance(project)
		val biomeServerService = project.service<BiomeServerService>()

		return panel {
			buttonsGroup {
				row {
					disabledConfiguration =
						radioButton(
							JavaScriptBundle.message(
								"settings.javascript.linters.autodetect.disabled",
								displayName
							)
						)
							.bindSelected(ConfigurationModeProperty(settings, ConfigurationMode.DISABLED))
							.component
				}
				row {
					automaticConfiguration =
						radioButton(
							JavaScriptBundle.message(
								"settings.javascript.linters.autodetect.configure.automatically",
								displayName
							)
						)
							.bindSelected(ConfigurationModeProperty(settings, ConfigurationMode.AUTOMATIC))
							.component

					val detectAutomaticallyHelpText = JavaScriptBundle.message(
						"settings.javascript.linters.autodetect.configure.automatically.help.text",
						ApplicationNamesInfo.getInstance().fullProductName,
						displayName,
						"biome.json"
					)

					val helpLabel = ContextHelpLabel.create(detectAutomaticallyHelpText)
					helpLabel.border = JBUI.Borders.emptyLeft(UIUtil.DEFAULT_HGAP)
					cell(helpLabel)
				}
				row {
					manualConfiguration =
						radioButton(
							JavaScriptBundle.message(
								"settings.javascript.linters.autodetect.configure.manually",
								displayName
							)
						)
							.bindSelected(ConfigurationModeProperty(settings, ConfigurationMode.MANUAL))
							.component
				}
			}
			panel {
				row(BiomeBundle.message("biome.path.label")) {
					textFieldWithBrowseButton(BiomeBundle.message("biome.path.label")) { fileChosen(it) }
						.bindText(settings::executablePath)
				}.visibleIf(manualConfiguration.selected)

				row(BiomeBundle.message("biome.config.path.label")) {
					textFieldWithBrowseButton(BiomeBundle.message("biome.config.path.label")) { fileChosen(it) }
						.bindText(settings::configPath)
				}.visibleIf(manualConfiguration.selected)
			}
			row(BiomeBundle.message("biome.run.format.for.files.label")) {
				runFormatForFilesField = textField()
					.align(AlignX.FILL)
					.bind(
						{ textField -> textField.text.trim() },
						JTextComponent::setText,
						MutableProperty({ settings.formatFilePattern }, { settings.formatFilePattern = it })
					)
					.validationOnInput {
						try {
							FileSystems.getDefault().getPathMatcher("glob:" + it.text)
							null
						} catch (e: PatternSyntaxException) {
							@NlsSafe val firstLine = e.localizedMessage?.lines()?.firstOrNull()
							ValidationInfo(firstLine ?: BiomeBundle.message("biome.invalid.pattern"), it)
						}
					}
					.component
			}.enabledIf(!disabledConfiguration.selected)

			row(BiomeBundle.message("biome.run.lint.for.files.label")) {
				runLintForFilesField = textField()
					.comment(BiomeBundle.message("biome.files.pattern.comment"))
					.align(AlignX.FILL)
					.bind(
						{ textField -> textField.text.trim() },
						JTextComponent::setText,
						MutableProperty({ settings.lintFilePattern }, { settings.lintFilePattern = it })
					)
					.validationOnInput {
						try {
							FileSystems.getDefault().getPathMatcher("glob:" + it.text)
							null
						} catch (e: PatternSyntaxException) {
							@NlsSafe val firstLine = e.localizedMessage?.lines()?.firstOrNull()
							ValidationInfo(firstLine ?: BiomeBundle.message("biome.invalid.pattern"), it)
						}
					}
					.component
			}.enabledIf(!disabledConfiguration.selected)

			row {
				runOnSaveCheckBox = checkBox(BiomeBundle.message("biome.run.format.on.save.label"))
					.bindSelected(RunOnObservableProperty(
						{ settings.configurationMode != ConfigurationMode.DISABLED && settings.formatOnSave },
						{ settings.formatOnSave = it },
						{ !disabledConfiguration.isSelected && runOnSaveCheckBox.isSelected }
					))
					.component

				val link = ActionsOnSaveConfigurable.createGoToActionsOnSavePageLink()
				cell(link)
			}.enabledIf(!disabledConfiguration.selected)

			onApply {
				biomeServerService.restartBiomeServer()
				biomeServerService.notifyRestart()
			}
		}

	}


	private fun fileChosen(file: VirtualFile): String {
		return file.path
	}

	private class ConfigurationModeProperty(
		private val settings: BiomeSettings,
		private val mode: ConfigurationMode
	) : MutableProperty<Boolean> {
		override fun get(): Boolean =
			settings.configurationMode == mode

		override fun set(value: Boolean) {
			if (value)
				settings.configurationMode = mode
		}
	}

	private inner class RunOnObservableProperty(
		private val getter: () -> Boolean,
		private val setter: (Boolean) -> Unit,
		private val afterConfigModeChangeGetter: () -> Boolean,
	) : ObservableMutableProperty<Boolean> {
		override fun set(value: Boolean) {
			setter(value)
		}

		override fun get(): Boolean =
			getter()

		override fun afterChange(parentDisposable: Disposable?, listener: (Boolean) -> Unit) {

			fun emitChange(radio: JBRadioButton) {
				if (radio.isSelected) {
					listener(afterConfigModeChangeGetter())
				}
			}

			manualConfiguration.whenItemSelected(parentDisposable, ::emitChange)
			automaticConfiguration.whenItemSelected(parentDisposable, ::emitChange)
			disabledConfiguration.whenItemSelected(parentDisposable, ::emitChange)
		}
	}

	companion object{
		 const val CONFIGURABLE_ID = "Settings.Biome"
	}
}
