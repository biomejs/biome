package com.github.biomejs.intellijbiome.settings

import com.intellij.openapi.components.BaseState
import com.intellij.openapi.components.Service
import org.jetbrains.annotations.ApiStatus

@Service
@ApiStatus.Internal
class BiomeSettingsState : BaseState() {
    var executablePath by string()
}
