---
title: 与 Prettier 的区别
description: 深入解释与 Prettier 的不同之处。
---

在某些情况下，Biome 主观决定以与 Prettier 输出不一致的方式来格式化代码。下文将解释这些差异。

## Prettier 不会取消引用某些作为有效 JavaScript 标识符的对象属性

Prettier 和 Biome 取消引用作为有效 JavaScript 标识符的对象和类属性。
Prettier [仅取消引号有效的 ES5 标识符](https://github.com/prettier/prettier/blob/a5d502513e5de4819a41fd90b9be7247146effc7/src/language-js/utils/index.js#L646).

这是 ES2015 现在广泛使用的生态系统中的遗留限制。
因此，我们决定通过在 ES2015+ 中取消引用所有有效的 JavaScript 标识符来实现不同。

一个可行的解决方法是引入一个配置来设置项目使用的 ECMAScript 版本。
然后我们可以根据该版本调整取消引用行为。
将 ECMAScript 版本设为 `ES5` 可以匹配 Prettier 的行为。

```js title="example.js"
const obj = {
 'a': true,
 b: true,
 "𐊧": true,
}
```

区别

```js title="example.js" del={4} ins={5}
const obj = {
  a: true,
  b: true,
  "𐊧": true,
  𐊧: true,
};
```


## Prettier 在计算键中的赋值行为不一致

Prettier 和 Biome 将一些赋值表达式括在括号之间，特别是在条件语句中。
这使得 Biome 能够识别应该进行比较的表达。

Prettier 的行为并不一致，因为它在对象属性的计算键赋值时添加括号，而在类属性的计算键赋值时不添加括号，如下例所示：

输入

```js title="example.js"
a = {
  [x = 0]: 1,
}

class C {
  [x = 0] = 1
}
```

区别

```js title="example.js" del={2} ins={3}
a = {
  [(x = 0)]: 1,
  [x = 0]: 1,
};

class C {
  [x = 0] = 1;
}
```

[Playground 链接](https://biomejs.dev/playground?enabledLinting=false&code=YQAgAD0AIAB7AAoAIAAgAFsAeAAgAD0AIAAwAF0AOgAgADEALAAKAH0ACgAKAGMAbABhAHMAcwAgAEMAIAB7AAoAIAAgACAAIABbAHgAIAA9ACAAMABdACAAPQAgADEACgB9AAoA)

为了保持一致，我们决定舍弃括号。
或者，我们也可以把任何赋值括在对象或类的计算键中。

## Prettier 为箭头函数的类型参数添加了逗号尾部，即使在不需要逗号尾部的情况下也是如此

在某些特定情况下，箭头函数的类型参数列表需要使用逗号来与 JSX 元素区分开来。
如果提供了默认类型，则不需要使用逗号。
在这里，我们偏离了 Prettier，因为我们认为这样更能尊重 Prettier 的初衷，即只有在需要时才添加尾部逗号。

输入

```tsx title="example.tsx"
<T = unknown>() => {};
```

区别

```tsx title="example.tsx" del={1} ins={2}
<T = unknown,>() => {};
<T = unknown>() => {};
```


## Prettier 对括号中的非空断言可选链的行为不一致

在 _TypeScript_ 中，非空断言操作符 `!` 允许断言一个值是非空的。
当应用于可选链时，断言适用于整个链，无论是否存在括号，
使 `(a.?.b)!` 和 `a.?.b!` 等同。

根据 Prettier，前面的代码示例已经格式化得很好了。
Prettier 用于强制括号的存在或不存在。
这看起来像是错失了标准化代码的机会。

此外，即使括号包含了非空断言，Prettier 也不会移除括号。
相反，它会将运算符移到括号外。

输入

```ts title="example.ts"
a.?.b!
(a.?.b)!
(a.?.b!)
```

区别

```ts title="example.ts" del={2, 4} ins={3, 5}
a.?.b!
(a.?.b)!
a.?.b!
(a.?.b)!
a.?.b!
```


## Prettier 格式化无效语法

Prettier 对 JavaScript 和 TypeScript 的基于 Babel 的解析非常宽松，允许忽略[多个错误](https://github.com/prettier/prettier/blob/e4a74c05f4502dd4ec70495c3130ff08ab088e05/src/language-js/parse/babel.js#L177-L218)。
Biome 的解析器有意比 Prettier 解析器更严格。
它能正确识别以下语法错误：

- 函数不能有重复的修改器
- 属性修饰符的顺序无效
- 函数声明不允许有主体
- 非抽象类不能有抽象属性
- 不能指定可选链
- 不能在接口的类型参数上设置 `const` 修饰符
- 顶层返回
- 等等...

在 Prettier 中，这些错误不被视为解析错误，AST 仍会 “正确” 地使用适当的节点构建。
在格式化时，Prettier 会将这些节点视为正常节点并进行相应格式化。

在 Biome 中，解析错误会导致 `Bogus` 节点，其中可能包含任意数量的有效节点、无效节点和/或原始字符。
在格式化时，Biome 会将假节点视为纯文本，将其逐字打印到生成的代码中，而不进行任何格式化，因为尝试格式化它们可能会导致错误和语义变化。

对于类属性，Prettier 当前的解析策略也使用布尔字段来表示修饰符，这意味着每种修饰符只能出现一个（可访问性修饰符存储为单个字符串）。
打印时，Prettier 会查看布尔字段列表，然后决定再次打印哪些修饰符。而 Biome 保存的是修饰符列表，这意味着重复的修饰符会被保留下来并被分析（因此会出现关于重复修饰符和排序的解析错误信息）。
在打印出假节点时，该列表将保持不变，而打印出未格式化的文本时，这些修饰符将继续存在。

Biome 有多种方法可以解决这个问题。
一种可能性是在格式化时尝试解释虚假节点并从中构造有效节点。
如果可以构建有效的节点，那么它只会像正常一样格式化该节点，否则，它会像当前一样逐字打印伪造的文本。
然而，这很混乱，并且向格式化程序引入了一种没有意义的解析逻辑形式。

另一种方法是在解析器中引入某种形式的“语法无效假节点”，它可以接受这类纯粹的语义错误（重复修饰符、非抽象类中的抽象属性）。

它将继续像正常一样构建节点（有效匹配 Prettier 中的行为），但会将节点存储在一种新的假节点中，包括诊断结果。
在格式化时，这些特定的假节点会尝试格式化内部节点，如果出现错误，就会回退（现有的 `format_or_verbatim` 工具已经可以做到这一点）。
这将使解析和格式化逻辑相互分离，但会给解析器带来更多复杂性，使无效状态被视为半有效状态。

### 类属性上的重复修饰符

输入

```ts title="example.ts"
// Multiple accessibility modifiers
class Foo {
  private public a  = 1;
}

// Declare function with body
declare function foo ( ) {  }

// Invalid use of abstract
class Bar {
  abstract  foo  ;
}

// Duplicate Readonly
class Read {
  readonly readonly   x: number;
}
```

区别

```ts title="example.ts" del={3, 8, 13, 19} ins={4, 9, 14, 20}
// Multiple accessibility modifiers
class Foo {
  private a = 1;
  private public a  = 1;
}

// Declare function with body
declare function foo() {};
declare function foo ( ) {  }

// Invalid use of abstract
class Bar {
  abstract foo;
  abstract  foo  ;
}

// Duplicate Readonly
class Read {
  readonly x: number;
  readonly readonly   x: number;
}
```

### 分配给一个可选链

输入

```js title="example.js"
(a?.b) = c;
```

区别

```js title="example.js" del={1} ins={2}
a?.b = c;
(a?.b) = c;
```

### 接口类型参数的修饰符不正确

输入

```ts title="example.js"
interface L<in const T> {}
```

区别

```ts title="example.js" del={1} ins={2}
interface L<const in T> {}
interface L<in const T> {}
```

### 顶层返回

```js title="example.js"
return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD
```

```js title="example.js" del={1, 2, 3, 4, 5, 6} ins={7}
return (
  someVeryLongStringA &&
  someVeryLongStringB &&
  someVeryLongStringC &&
  someVeryLongStringD
);
return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD
```

### 错误的自增和自减

输入

```js title="example.js"
(1)++;
```

```js title="example.js" del={1} add={2}
1++;
(1)++;
```

### 在非抽象类中使用 `abstract` 修饰符

输入

```ts title="example.js"
class C {
  abstract f() : number;
}
```

区别


```ts title="example.js" del={2} add={3}
class C {
  abstract f(): number;
  abstract f() : number;
}
```

## Prettier 在 TypeScript 和 Babel 解析之间存在不一致问题

Prettier 支持多种不同的 JavaScript 和 TypeScript 代码解析器，所有这些解析器都旨在与 [`estree` 规范](https://github.com/estree/estree)兼容。 大多数情况下，Prettier 使用 Babel 作为 JavaScript 代码的默认解析器，但在解析 TypeScript 时，它会首先尝试使用 TypeScript 自带的解析器，然后在启用 TypeScript 后才返回 Babel。虽然 TypeScript 解析器通常与 estree 兼容，但它并不精确，这[可能会导致一些不一致](https://github.com/prettier/prettier/issues/15785)，从而影响 Prettier 创建的输出。一般来说，这些被认为是 Prettier 本身的错误，因为无论使用哪个解析器，输出都应该是相同的。

Biome 实现了自己的解析，可以处理所有形式的 JavaScript 和 TypeScript 代码，这意味着两者之间不应该存在任何不一致。不过，在将 TypeScript 代码库从 Prettier 迁移到 Biome 时，可能会因为 Prettier 解析器之间的差异而导致某些格式发生变化。

这些情况在 Biome 中不被视为错误或不兼容。如果格式化的代码在使用 Prettier 中的 typescript 解析器设置时显示不同，但在使用 babel 和/或 babel-ts 时匹配，则 Biome 认为输出是兼容的。

作为一个例子，考虑这种情况，使用 Biome 和 Prettier 3.1.0 以及 `typescript` 解析器进行格式化：

输入

```ts title="example.js"
function someFunctionName(
  someLongBreakingParameterName,
  anotherLongParameterName,
) {
  return isEqual(a?.map(([t, _]) => t?.id), b?.map(([t, _]) => t?.id));
}
```

区别

```ts title="example.js" del={5} ins={6,7,8,9}
function someFunctionName(
  someLongBreakingParameterName,
  anotherLongParameterName,
) {
  return isEqual(a?.map(([t, _]) => t?.id), b?.map(([t, _]) => t?.id));
  return isEqual(
    a?.map(([t, _]) => t?.id),
    b?.map(([t, _]) => t?.id),
  );
}
```

使用 TypeScript 解析器的 Prettier 选择在一行上编写 `isEqual` 调用，而 Biome 将 Prettier 的输出与 `babel` 和 `babel-ts` 解析器进行匹配。因此，这不被视为与 Biome 不兼容，而是被视为 Prettier 中的错误。
