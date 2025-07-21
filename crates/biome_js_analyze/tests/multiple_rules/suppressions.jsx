///! lint/a11y/useKeyWithClickEvents
///! lint/a11y/useSemanticElements
///! lint/correctness/noChildrenProp

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

function firstDisabledStar() {
  return (
    /* biome-ignore lint/a11y/useKeyWithClickEvents: ... */
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

function bothDisabledMixed1() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    /* biome-ignore lint/a11y/useSemanticElements: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledMixed2() {
  return (
    /* biome-ignore lint/a11y/useKeyWithClickEvents: ... */
    // biome-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function onlyLastDisabledWithSpacing() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...

    // biome-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledFarAway() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint/a11y/useSemanticElements: ...
    // biome-ignore lint/a11y/noRedundantAlt: ...
    // biome-ignore lint/security/noBlankTarget: ...
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

function wideThenNarrow() {
  return (
    // biome-ignore lint: ...
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function wideThenNarrowUnused() {
  return (
    // biome-ignore lint: ...
    // biome-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowThenWide() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowThenWideUnused() {
  return (
    // biome-ignore lint/a11y/useKeyWithClickEvents: ...
    // biome-ignore lint/a11y/useSemanticElements: ...
    // biome-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowUnusedThenWide() {
  return (
    // biome-ignore lint/security/noBlankTarget: ...
    // biome-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function tagAndAttribute1() {
  return (
    // biome-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      // biome-ignore lint/correctness/noChildrenProp: ...
      children={[]}
    >Some text</span>
  )
}

function tagAndAttribute2() {
  return (
    // biome-ignore lint/a11y/useSemanticElements: ...
    // biome-ignore lint/correctness/noChildrenProp: ...
    <span
      role="button"
      children={[]}
    >Some text</span>
  )
}

function bothDisabledInsideJsx() {
  return (<>
    {/* biome-ignore lint/a11y/useKeyWithClickEvents: ... */}
    {/* biome-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledInsideJsxSameToken() {
  return (<>
    {/* biome-ignore lint/a11y/useKeyWithClickEvents: ... */
    /* biome-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledViaSameComment() {
  return (
    /*biome-ignore lint/a11y/useKeyWithClickEvents: ...
    biome-ignore lint/a11y/useSemanticElements: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledViaSameComment2() {
  return (
    /*
    biome-ignore lint/a11y/useKeyWithClickEvents: ...
    biome-ignore lint/a11y/useSemanticElements: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledViaSameCommentInJsx() {
  return (<>
    {/*
    biome-ignore lint/a11y/useKeyWithClickEvents: ...
    biome-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledViaSameCommentInJsx2() {
  return (<>
    {/*
    biome-ignore lint/a11y/useKeyWithClickEvents: ...
    biome-ignore lint/a11y/useSemanticElements: ... */ <span
      role="button"
      onClick={()=>null}
    >Some text</span>}
  </>)
}

// Four examples below are still imperfect - the whole comment is reported unused.
// That is probably good enough - suppression still works as intended, but we
// don't have access to precise ranges of each part.

function unusedCaughtWithinSameComment() {
  return (
    /*
    biome-ignore lint/a11y/useKeyWithClickEvents: ...
    biome-ignore lint/security/noBlankTarget: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameComment2() {
  return (
    /*
    biome-ignore lint/security/noBlankTarget: ...
    biome-ignore lint/a11y/useKeyWithClickEvents: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameComment3() {
  return (
    /* biome-ignore lint/security/noBlankTarget: ...
    biome-ignore lint/a11y/useKeyWithClickEvents: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameCommentInJsx() {
  return (<>
    {/* biome-ignore lint/security/noBlankTarget: ...
    biome-ignore lint/a11y/useKeyWithClickEvents: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}
