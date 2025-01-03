let obj = {};
<a {...obj, other} />;
<a ...obj} />;
<a {obj} />;
<div
      {...{} /*
      // @ts-ignore */ /* prettier-ignore */
      invalidProp="HelloWorld"
    />;
