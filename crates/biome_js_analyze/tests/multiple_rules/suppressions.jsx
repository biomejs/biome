///! lint/a11y/useKeyWithClickEvents
///! lint/a11y/useSemanticElements

function bothFailing() {
  return (
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function firstDisabled() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function secondDisabled() {
  return (
    // biome-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabled() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused1() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint/a11y/noRedundantAlt: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused2() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused3() {
  return (
    // biome-ignore lint/style/noImplicitBoolean: ...
    // biome-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}