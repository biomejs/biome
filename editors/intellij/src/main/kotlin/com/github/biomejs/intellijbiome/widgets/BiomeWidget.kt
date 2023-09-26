package com.github.biomejs.intellijbiome.widgets

import com.github.biomejs.intellijbiome.BiomeBundle
import com.github.biomejs.intellijbiome.listeners.BIOME_CONFIG_RESOLVED_TOPIC
import com.github.biomejs.intellijbiome.listeners.BiomeConfigResolvedListener
import com.github.biomejs.intellijbiome.lsp.BiomeLspServerSupportProvider
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.application.ModalityState
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.CustomStatusBarWidget
import com.intellij.openapi.wm.StatusBarWidget.WidgetPresentation
import com.intellij.openapi.wm.WindowManager
import com.intellij.openapi.wm.impl.status.EditorBasedWidget
import com.intellij.openapi.wm.impl.status.TextPanel.WithIconAndArrows
import com.intellij.platform.lsp.api.LspServerManager
import com.intellij.platform.lsp.impl.LspServerImpl
import javax.swing.JComponent

class BiomeWidget(project: Project) : EditorBasedWidget(project), CustomStatusBarWidget, WidgetPresentation {
	private var component: WithIconAndArrows? = null

	init {
		project
			.messageBus
			.connect(this)
			.subscribe(BIOME_CONFIG_RESOLVED_TOPIC, object : BiomeConfigResolvedListener {
				override fun resolved(version: String) {
					update("Biome $version")
				}
			})
	}

	override fun ID(): String {
		return "BiomeWidget"
	}

	override fun getPresentation(): WidgetPresentation {
		return this
	}

	override fun getComponent(): JComponent {
		val component = WithIconAndArrows()
		component.text = "Biome"
		component.toolTipText = getTooltipText()

		this.component = component

		return component
	}

	override fun getTooltipText(): String {
		val lspServerManager = LspServerManager.getInstance(project)
		val lspServer = lspServerManager.getServersForProvider(BiomeLspServerSupportProvider::class.java).firstOrNull()

		return when (lspServer) {
			is LspServerImpl -> {
				if (lspServer.isRunning) {
					BiomeBundle.message("biome.language.server.is.running")
				} else {
					BiomeBundle.message("biome.language.server.is.stopped")
				}
			}

			else -> {
				BiomeBundle.message("biome.language.server.is.stopped")
			}
		}
	}

	private fun update(text: String?) {
		ApplicationManager.getApplication()
			.invokeLater(
				{
					if (project.isDisposed || component == null) {
						return@invokeLater
					}

					component!!.text = text
					val statusBar = WindowManager.getInstance().getStatusBar(project)
					statusBar?.component?.updateUI()
				},
				ModalityState.any()
			)
	}

}
