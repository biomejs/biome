/* should not generate diagnostics */

function Component() {
  return <div />;
};

function someFunc() {
  if (condition) {
    return 5;
  }
  return 10;
}

function notAComponent() {
  if (condition) {
    return <div />;
  }
  return <div />;
}

callback(() => {
  if (condition) {
    return <div />;
  }
  return <div />;
});

function Component() {
  const renderContent = () => {
    if (false) return <></>;
    return <></>;
  };
  return <>{renderContent()}</>;
}

function Component() {
  function renderContent() {
    if (false) return <></>;
    return <></>;
  }
  return <>{renderContent()}</>;
}

function Component() {
  const renderContent = () => {
    const renderContentInner = () => {
      // ifs in render functions are fine no matter what nesting level this is
      if (false) return;
      return <></>;
    };
    return <>{renderContentInner()}</>;
  };
  return <></>;
}