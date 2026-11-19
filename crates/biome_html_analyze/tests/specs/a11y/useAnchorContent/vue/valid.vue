<!-- should not generate diagnostics -->
<template>
  <!-- Custom components should not trigger the rule -->
  <Link></Link>
  <CustomAnchor></CustomAnchor>
  <RouterLink></RouterLink>

  <!-- Native anchors with accessible content -->
  <a>content</a>
  <a>{{ stringVariable }}</a>
  <a><span>content</span></a>
  <a><span aria-hidden="true"></span>content</a>
  <a><div aria-hidden="true"></div>content</a>
  <a aria-hidden="false">content</a>
  <a><span aria-hidden="false">content</span></a>
  <a><img alt="description" /></a>
  <a title="Home">Home</a>
  <!-- Labels supplement accessible content -->
  <a aria-label="Navigate to dashboard">Dashboard</a>
  <a title="Go to settings page">Settings</a>

  <!-- Custom components may render accessible content -->
  <a><Image alt="description" /></a>
  <a><CustomIcon /></a>
</template>
