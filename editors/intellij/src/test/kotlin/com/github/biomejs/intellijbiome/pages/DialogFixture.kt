package com.github.biomejs.intellijbiome.pages

import com.intellij.remoterobot.RemoteRobot
import com.intellij.remoterobot.data.RemoteComponent
import com.intellij.remoterobot.fixtures.CommonContainerFixture
import com.intellij.remoterobot.fixtures.ContainerFixture
import com.intellij.remoterobot.fixtures.FixtureName
import com.intellij.remoterobot.search.locators.byXpath
import com.intellij.remoterobot.stepsProcessing.step
import java.time.Duration

fun ContainerFixture.dialog(
	title: String,
	timeout: Duration = Duration.ofSeconds(20),
	function: DialogFixture.() -> Unit = {}): DialogFixture = step("Search for dialog with title $title") {
	find<DialogFixture>(DialogFixture.byTitle(title), timeout).apply(function)
}

@FixtureName("Dialog")
class DialogFixture(
	remoteRobot: RemoteRobot,
	remoteComponent: RemoteComponent) : CommonContainerFixture(remoteRobot, remoteComponent) {

	companion object {
		@JvmStatic
		fun byTitle(title: String) = byXpath("title $title", "//div[@title='$title' and @class='MyDialog']")
	}

	val title: String
		get() = callJs("component.getTitle();")
}
