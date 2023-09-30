package com.github.biomejs.intellijbiome.listeners

import com.intellij.util.messages.Topic
import java.util.*


val BIOME_CONFIG_RESOLVED_TOPIC = Topic.create(
    "Biome config resolved topic",
    BiomeConfigResolvedListener::class.java
)

interface BiomeConfigResolvedListener : EventListener {
    fun resolved(version: String)
}
