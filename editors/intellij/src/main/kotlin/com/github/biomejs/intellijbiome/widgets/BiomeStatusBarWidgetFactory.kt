package com.github.biomejs.intellijbiome.widgets

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.openapi.wm.StatusBarWidgetFactory

class BiomeStatusBarWidgetFactory : StatusBarWidgetFactory {
    override fun getId(): String {
        return "BiomeWidget"
    }

    override fun getDisplayName(): String {
        return "Biome"
    }

    override fun createWidget(project: Project): StatusBarWidget {
        return BiomeWidget(project)
    }
}
