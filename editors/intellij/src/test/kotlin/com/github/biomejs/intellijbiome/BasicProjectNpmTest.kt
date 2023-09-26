package com.github.biomejs.intellijbiome

import com.github.biomejs.intellijbiome.pages.*
import com.github.biomejs.intellijbiome.utils.RemoteRobotExtension
import com.github.biomejs.intellijbiome.utils.StepsLogger
import com.intellij.remoterobot.RemoteRobot
import com.intellij.remoterobot.fixtures.ComponentFixture
import com.intellij.remoterobot.stepsProcessing.step
import com.intellij.remoterobot.utils.keyboard
import com.intellij.remoterobot.utils.waitFor
import com.intellij.remoterobot.utils.waitForIgnoringError
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import java.awt.Point
import java.awt.event.KeyEvent.*
import java.io.File
import java.time.Duration
import java.time.Duration.ofMinutes


@ExtendWith(RemoteRobotExtension::class)
class BasicProjectNpmTest {
    private val basicProjectPath = File("src/test/testData/basic-project")

    init {
        StepsLogger.init()
    }

    @BeforeEach
    fun waitForIde(remoteRobot: RemoteRobot) {
        waitForIgnoringError(ofMinutes(3)) { remoteRobot.callJs("true") }
    }

    @AfterEach
    fun closeProject(remoteRobot: RemoteRobot) = with(remoteRobot) {
        idea {
            if (remoteRobot.isMac()) {
                keyboard {
                    hotKey(VK_SHIFT, VK_META, VK_A)
                    enterText("Close Project", 20)
                    enter()
                }
            } else {
                menuBar.select("File", "Close Project")
            }
        }
    }

    @Test
    fun openQuickFixes(remoteRobot: RemoteRobot) = with(remoteRobot) {
        welcomeFrame {
            openProjectLink.click()
            dialog("Open File or Project") {
                directoryPath.text = basicProjectPath.absolutePath
                button("OK").click()
            }
        }

        idea {
            step("Check biome running version") {

                waitFor(ofMinutes(5)) { isDumbMode().not() }

                openFile("index.js")

                val editor = editor("index.js")

                editor.click(Point(0, 0))

                keyboard {
                    hotKey(VK_ALT, VK_ENTER)
                }

                quickfix {
                    val items = collectItems()

                    assertTrue(items.contains("Use 'const' instead."))
                    assertTrue(items.contains("Suppress rule lint/style/noVar"))
                }

                keyboard {
                    hotKey(VK_ESCAPE)
                }
            }
        }

    }
}
