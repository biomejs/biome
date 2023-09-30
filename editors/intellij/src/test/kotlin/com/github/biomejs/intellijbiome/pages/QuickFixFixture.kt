package com.github.biomejs.intellijbiome.pages

import com.intellij.remoterobot.fixtures.ContainerFixture
import com.intellij.remoterobot.fixtures.JListFixture
import com.intellij.remoterobot.search.locators.byXpath
import com.intellij.remoterobot.stepsProcessing.step
import java.time.Duration

fun ContainerFixture.quickfix(
    timeout: Duration = Duration.ofSeconds(20),
    function: JListFixture.() -> Unit = {}
): JListFixture = step("Search for quickfixes panel") {
    find<JListFixture>(byXpath("//div[@class='MyList']"), timeout).apply(function)
}
