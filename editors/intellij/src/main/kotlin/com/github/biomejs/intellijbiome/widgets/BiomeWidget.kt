package com.github.biomejs.intellijbiome.widgets

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.openapi.wm.impl.status.EditorBasedWidget
class BiomeWidget(myProject: Project) : EditorBasedWidget(myProject) {

    override fun ID(): String {
        return "BiomeWidget"
    }

    override fun getPresentation(): StatusBarWidget.WidgetPresentation {
        return BiomeWidgetPresentation(project)
    }
}