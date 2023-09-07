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

    companion object {
        @JvmStatic
        fun getInstance(project: Project): BiomeSettings = project.service()
    }
}
