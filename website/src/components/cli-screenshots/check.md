<pre class="language-text homepage-example collapsed collapsable"><code class="language-text"><span style="color: CornflowerBlue">$</span> biome check

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">src/App.jsx:12:3</span> <strong>lint/jsx-a11y/altText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;">elements.</span>

  <strong>  10</strong><strong> │ </strong>  <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App&quot;</span>&gt;
  <strong>  11</strong><strong> │ </strong>    &lt;<span class="token attr-name">header</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App-header&quot;</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 12</strong><strong> │ </strong>      &lt;<span class="token attr-name">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">logo2</span><span class="token punctuation">}</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App-logo&quot;</span> <span class="token operator">/</span>&gt;
      <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  13</strong><strong> │ </strong>      &lt;<span class="token attr-name">p</span>&gt;
  <strong>  14</strong><strong> │ </strong>        Edit

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: DodgerBlue;">readers to understand content&apos;s purpose within a page.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">src/App.jsx:12:13</span> <strong>lint/js/noUndeclaredVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>logo2</strong></span><span style="color: Tomato;"> variable is undeclared</span>

  <strong>  10</strong><strong> │ </strong>  <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App&quot;</span>&gt;
  <strong>  11</strong><strong> │ </strong>    &lt;<span class="token attr-name">header</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App-header&quot;</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 12</strong><strong> │ </strong>      &lt;<span class="token attr-name">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">logo2</span><span class="token punctuation">}</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;App-logo&quot;</span> <span class="token operator">/</span>&gt;
      <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  13</strong><strong> │ </strong>      &lt;<span class="token attr-name">p</span>&gt;
  <strong>  14</strong><strong> │ </strong>        Edit

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Did you mean </span><span style="color: DodgerBlue;"><strong>logo</strong></span><span style="color: DodgerBlue;">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">logo</span><span style="color: Tomato;"><strong>2</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">logo</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">src/App.jsx:2:7</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The import variable </span><span style="color: Tomato;"><strong>logo</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// @jsx</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">logo</span> <span class="token keyword">from</span> <span class="token string">&quot;./logo.svg&quot;</span><span class="token punctuation">;</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token string">&quot;./App.css&quot;</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Found </span><span style="color: Tomato;"><strong>3</strong></span><span style="color: Tomato;"> </span><span style="color: Tomato;">problems</span></code><button aria-hidden="true" class="expand">Click to Expand</button></pre>
