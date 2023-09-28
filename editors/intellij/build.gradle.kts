import java.net.URI

fun properties(key: String) = providers.gradleProperty(key)
fun environment(key: String) = providers.environmentVariable(key)
val remoteRobotVersion = "0.11.20"

plugins {
    id("java") // Java support
    alias(libs.plugins.kotlin) // Kotlin support
    alias(libs.plugins.gradleIntelliJPlugin) // Gradle IntelliJ Plugin
}

group = properties("pluginGroup").get()
version = properties("pluginVersion").get()

// Configure project's dependencies
repositories {
    mavenCentral()
		maven { url = URI("https://packages.jetbrains.team/maven/p/ij/intellij-dependencies") }
}

// Dependencies are managed with Gradle version catalog - read more: https://docs.gradle.org/current/userguide/platforms.html#sub:version-catalog
dependencies {
	testImplementation("com.intellij.remoterobot:remote-robot:$remoteRobotVersion")
	testImplementation("com.intellij.remoterobot:remote-fixtures:$remoteRobotVersion")
	testImplementation("org.junit.jupiter:junit-jupiter-api:5.9.3")
	testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.9.2")
	testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.9.3")

	// Logging Network Calls
	testImplementation("com.squareup.okhttp3:logging-interceptor:4.11.0")

	// Video Recording
	implementation("com.automation-remarks:video-recorder-junit5:2.0")
}

// Set the JVM language level used to build the project. Use Java 11 for 2020.3+, and Java 17 for 2022.2+.
kotlin {
    jvmToolchain(17)
}

// Configure Gradle IntelliJ Plugin - read more: https://plugins.jetbrains.com/docs/intellij/tools-gradle-intellij-plugin.html
intellij {
    pluginName = properties("pluginName")
    version = properties("platformVersion")
    type = properties("platformType")

    // Plugin Dependencies. Uses `platformPlugins` property from the gradle.properties file.
    plugins = properties("platformPlugins").map { it.split(',').map(String::trim).filter(String::isNotEmpty) }
}

tasks {
    wrapper {
        gradleVersion = properties("gradleVersion").get()
    }

    patchPluginXml {
        version = properties("pluginVersion")
        sinceBuild = properties("pluginSinceBuild")
        untilBuild = properties("pluginUntilBuild")
    }

		downloadRobotServerPlugin {
			version.set(remoteRobotVersion)
		}

	// Configure UI tests plugin
    // Read more: https://github.com/JetBrains/intellij-ui-test-robot
    runIdeForUiTests {
        systemProperty("robot-server.port", "8082")
        systemProperty("ide.mac.message.dialogs.as.sheets", "false")
        systemProperty("jb.privacy.policy.text", "<!--999.999-->")
        systemProperty("jb.consents.confirmation.enabled", "false")
				systemProperty("ide.mac.file.chooser.native", "false")
				systemProperty("jbScreenMenuBar.enabled", "false")
				systemProperty("apple.laf.useScreenMenuBar", "false")
				systemProperty("idea.trust.all.projects", "true")
				systemProperty("ide.show.tips.on.startup.default.value", "false")
				systemProperty("eap.require.license", "false")

    }

		test {
			useJUnitPlatform()
		}

    signPlugin {
        certificateChain = environment("CERTIFICATE_CHAIN")
        privateKey = environment("PRIVATE_KEY")
        password = environment("PRIVATE_KEY_PASSWORD")
    }

    publishPlugin {
        token = environment("PUBLISH_TOKEN")
        // The pluginVersion is based on the SemVer (https://semver.org) and supports pre-release labels, like 2.1.7-alpha.3
        // Specify pre-release label to publish the plugin in a custom Release Channel automatically. Read more:
        // https://plugins.jetbrains.com/docs/intellij/deployment.html#specifying-a-release-channel
        channels = properties("pluginVersion").map { listOf(it.split('-').getOrElse(1) { "default" }.split('.').first()) }
    }
}



