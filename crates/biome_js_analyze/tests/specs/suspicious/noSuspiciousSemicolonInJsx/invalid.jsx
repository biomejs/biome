const Component = () => {
    return (
        <div>
          <div />;
        </div>
    );
}

const Component2 = () => {
    return (
        <div>
          <Component>
            <div />
          </Component>;
        </div>
    );
}

const Component3 = () => (
    <div>
        <Component />;
    </div>
)

const Component4 = () => {
  return (
      <>
          <div />;
      </>
  );
}

const Component5 = () => {
  return (
      <>
        <Component>
          <div />
        </Component>;
      </>
  );
}
