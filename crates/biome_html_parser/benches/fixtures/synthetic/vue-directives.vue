<template>
  <div v-if="showMain" class="main-container">
    <header v-if="showHeader" v-bind:class="headerClass" :style="headerStyle">
      <h1 v-if="title">{{ title }}</h1>
      <h2 v-else-if="subtitle">{{ subtitle }}</h2>
      <h3 v-else>Default Header</h3>
    </header>
    
    <nav v-if="showNav" v-bind:id="navId" :data-active="isActive">
      <ul v-if="hasItems">
        <li v-for="item in items" :key="item.id" v-bind:class="item.class" :data-index="item.index">
          <a v-if="item.link" :href="item.link" v-on:click="handleClick" @mouseenter="handleHover" @mouseleave="handleLeave">{{ item.name }}</a>
          <span v-else-if="item.disabled" class="disabled">{{ item.name }}</span>
          <span v-else>{{ item.name }}</span>
        </li>
      </ul>
      <p v-else>No items available</p>
    </nav>
    
    <main v-if="showContent" v-bind:class="contentClass">
      <section v-if="sectionA" v-for="(section, idx) in sections" :key="idx" v-bind:id="section.id" :data-section="section.type">
        <h2 v-if="section.title">{{ section.title }}</h2>
        <h3 v-else-if="section.subtitle">{{ section.subtitle }}</h3>
        <h4 v-else>Untitled Section</h4>
        
        <article v-for="article in section.articles" :key="article.id" v-bind:class="article.class" :data-article="article.type">
          <p v-if="article.content">{{ article.content }}</p>
          <p v-else-if="article.summary">{{ article.summary }}</p>
          <p v-else>No content</p>
        </article>
      </section>
      
      <aside v-if="showSidebar" v-bind:class="sidebarClass" :data-visible="sidebarVisible">
        <div v-for="widget in widgets" :key="widget.id" v-bind:class="widget.class" :data-widget="widget.type">
          <h4 v-if="widget.title">{{ widget.title }}</h4>
          <h5 v-else-if="widget.label">{{ widget.label }}</h5>
          <h6 v-else>Widget</h6>
          
          <p v-if="widget.text">{{ widget.text }}</p>
          <p v-else-if="widget.description">{{ widget.description }}</p>
          <p v-else>No description</p>
        </div>
      </aside>
    </main>
    
    <footer v-if="showFooter" v-bind:class="footerClass" :data-footer="footerType">
      <div v-for="link in footerLinks" :key="link.id" v-bind:class="link.class" :data-link="link.type">
        <a v-if="link.url" :href="link.url" v-on:click.prevent="handleFooterClick" @mouseover="handleFooterHover">{{ link.text }}</a>
        <span v-else-if="link.text">{{ link.text }}</span>
        <span v-else>Link</span>
      </div>
    </footer>
  </div>
  
  <div v-else-if="showAlternative" class="alternative-container">
    <div v-for="alt in alternatives" :key="alt.id" v-bind:class="alt.class" :data-alt="alt.type">
      <h2 v-if="alt.title">{{ alt.title }}</h2>
      <h3 v-else-if="alt.heading">{{ alt.heading }}</h3>
      <h4 v-else>Alternative</h4>
      
      <p v-if="alt.content">{{ alt.content }}</p>
      <p v-else-if="alt.body">{{ alt.body }}</p>
      <p v-else>No content</p>
    </div>
  </div>
  
  <div v-else class="empty-container">
    <p>No content to display</p>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const showMain = ref(true)
const showHeader = ref(true)
const showNav = ref(true)
const showContent = ref(true)
const showSidebar = ref(true)
const showFooter = ref(true)
const showAlternative = ref(false)
const sectionA = ref(true)

const title = ref('Main Title')
const subtitle = ref('Subtitle')
const headerClass = ref('header')
const headerStyle = ref({ color: 'blue' })
const navId = ref('main-nav')
const isActive = ref(true)
const hasItems = ref(true)
const items = ref([
  { id: 1, name: 'Home', link: '/', class: 'nav-item', index: 0 },
  { id: 2, name: 'About', link: '/about', class: 'nav-item', index: 1 },
  { id: 3, name: 'Contact', link: '/contact', class: 'nav-item', index: 2 }
])

const contentClass = ref('content')
const sections = ref([
  {
    id: 'sec1',
    type: 'primary',
    title: 'Section One',
    articles: [
      { id: 'a1', class: 'article', type: 'text', content: 'Article content' },
      { id: 'a2', class: 'article', type: 'summary', summary: 'Article summary' }
    ]
  }
])

const sidebarClass = ref('sidebar')
const sidebarVisible = ref(true)
const widgets = ref([
  { id: 'w1', class: 'widget', type: 'info', title: 'Widget 1', text: 'Widget text' },
  { id: 'w2', class: 'widget', type: 'alert', label: 'Widget 2', description: 'Widget description' }
])

const footerClass = ref('footer')
const footerType = ref('main')
const footerLinks = ref([
  { id: 'f1', class: 'footer-link', type: 'internal', url: '/privacy', text: 'Privacy' },
  { id: 'f2', class: 'footer-link', type: 'external', url: '/terms', text: 'Terms' }
])

const alternatives = ref([
  { id: 'alt1', class: 'alt-item', type: 'option', title: 'Option 1', content: 'Content 1' },
  { id: 'alt2', class: 'alt-item', type: 'option', heading: 'Option 2', body: 'Content 2' }
])

const handleClick = () => console.log('click')
const handleHover = () => console.log('hover')
const handleLeave = () => console.log('leave')
const handleFooterClick = () => console.log('footer click')
const handleFooterHover = () => console.log('footer hover')
</script>

<style scoped>
.main-container { padding: 20px; }
.header { background: #f0f0f0; }
.nav-item { display: inline-block; }
.content { margin: 20px 0; }
.sidebar { float: right; }
.footer { clear: both; }
</style>
