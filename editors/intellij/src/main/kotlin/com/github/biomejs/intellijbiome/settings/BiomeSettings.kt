package com.github.biomejs.intellijbiome.settings

import com.intellij.openapi.components.*
import com.intellij.openapi.project.Project


@Service(Service.Level.PROJECT)
@State(name = "BiomeSettings", storages = [(Storage("biome.xml"))])
class BiomeSettings :
	SimplePersistentStateComponent<BiomeSettingsState>(BiomeSettingsState()) {
	var executablePath: String
		get() = state.executablePath ?: ""
		set(value) {
			state.executablePath = value
		}
	var configPath: String
		get() = state.configPath ?: ""
		set(value) {
			state.configPath = value
		}

	var formatFilePattern: String
		get() = state.formatFilePattern ?: BiomeSettingsState.DEFAULT_FILE_PATTERN
		set(value) {
			state.formatFilePattern = value
		}

	var lintFilePattern: String
		get() = state.lintFilePattern ?: BiomeSettingsState.DEFAULT_FILE_PATTERN
		set(value) {
			state.lintFilePattern = value
		}

	var configurationMode: ConfigurationMode
		get() = state.configurationMode
		set(value) {
			state.configurationMode = value
		}

	var formatOnSave: Boolean
		get() = state.formatOnSave
		set(value) {
			state.formatOnSave = value
		}

	fun formatOnSave(): Boolean {
		return !isDisabled() && formatOnSave
	}

	private fun isDisabled(): Boolean {
		return configurationMode === ConfigurationMode.DISABLED
	}

	companion object {
		@JvmStatic
		fun getInstance(project: Project): BiomeSettings = project.service()
	}
}
