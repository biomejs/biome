package com.github.biomejs.intellijbiome.settings

import com.intellij.openapi.components.BaseState
import com.intellij.openapi.components.Service
import com.jetbrains.rd.generator.nova.PredefinedType
import org.jetbrains.annotations.ApiStatus

@ApiStatus.Internal
enum class ConfigurationMode {
	DISABLED,
	AUTOMATIC,
	MANUAL
}

@Service
@ApiStatus.Internal
class BiomeSettingsState : BaseState() {
    var executablePath by string()
    var configPath by string()
	var formatFilePattern by string(DEFAULT_FILE_PATTERN)
	var lintFilePattern  by string(DEFAULT_FILE_PATTERN)
	var formatOnSave by property(false)
	var configurationMode by enum(ConfigurationMode.AUTOMATIC)

	companion object {
		const val DEFAULT_FILE_PATTERN = "**/*.{js,mjs,cjs,ts,jsx,tsx,cts,json,jsonc}"
	}
}
