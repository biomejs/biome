package com.github.biomejs.intellijbiome.formatter

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.settings.BiomeConfigurable
import com.github.biomejs.intellijbiome.settings.BiomeSettings
import com.intellij.ide.actionsOnSave.*

class BiomeOnSaveInfoProvider : ActionOnSaveInfoProvider() {
	override fun getActionOnSaveInfos(context: ActionOnSaveContext):
		List<ActionOnSaveInfo> = listOf(BiomeOnSaveActionInfo(context))

	override fun getSearchableOptions(): Collection<String> {
		return listOf(BiomeBundle.message("biome.run.format.on.save.checkbox.on.actions.on.save.page"))
	}
}


private class BiomeOnSaveActionInfo(actionOnSaveContext: ActionOnSaveContext)
	: ActionOnSaveBackedByOwnConfigurable<BiomeConfigurable>(actionOnSaveContext, BiomeConfigurable.CONFIGURABLE_ID, BiomeConfigurable::class.java) {

	override fun getActionOnSaveName() = BiomeBundle.message("biome.run.format.on.save.checkbox.on.actions.on.save.page")

	override fun isActionOnSaveEnabledAccordingToStoredState() = BiomeSettings.getInstance(project).formatOnSave

	override fun isActionOnSaveEnabledAccordingToUiState(configurable: BiomeConfigurable) = configurable.runOnSaveCheckBox.isSelected

	override fun setActionOnSaveEnabled(configurable: BiomeConfigurable, enabled: Boolean) {
		configurable.runOnSaveCheckBox.isSelected = enabled
	}

	override fun getActionLinks() = listOf(createGoToPageInSettingsLink(BiomeConfigurable.CONFIGURABLE_ID))
}
