---
title: Prettier との違い
description: Prettier との違いを深く解説
---

Prettier との間にいくつかの相違点が存在します。

### Prettierは有効なJavaScript識別子である一部のオブジェクトプロパティの引用符を外しません

PrettierとBiomeは、有効なJavaScript識別子であるオブジェクトおよびクラスプロパティの引用符を外します。
Prettierは、特に[ES5において有効な識別子のみ引用符を外します](https://github.com/prettier/prettier/blob/a5d502513e5de4819a41fd90b9be7247146effc7/src/language-js/utils/index.js#L646)。

ES2015が広まっている現在のエコシステムにおいて、これは古い制限です。
そのため、私たちはES2015以降の全ての有効なJavaScript識別子の引用符を外すことにしました。

プロジェクトが使用するECMAScriptバージョンを設定するための設定を導入することは可能な解決策かもしれません。
そのバージョンに基づいて引用符を外す動作を調整できます。
ECMAScriptのバージョンをES5に設定することで、Prettierの挙動に一致させることができます。

```js title="example.js"
const obj = {
 'a': true,
 b: true,
 "𐊧": true,
}
```

Diff

```js title="exmaple.js" del={4} ins={5}
const obj = {
  a: true,
  b: true,
  "𐊧": true,
  𐊧: true,
};
```


### Prettierは計算プロパティでの代入で一貫性のない挙動を示します

PrettierとBiomeは、特に条件文などで代入式を括弧で囲みます。
これにより、Biomeは比較式であるべきコードを識別できます。

Prettierは、オブジェクトのプロパティの計算されたキーの割り当てに括弧を追加する一方で、クラスのプロパティの計算されたキーではそれを行いません。以下の例で示されます：

入力

```js title="example.js"
a = {
  [x = 0]: 1,
}

class C {
  [x = 0] = 1
}
```

差分

```js title="example.js" del={2} ins={3}
a = {
  [(x = 0)]: 1,
  [x = 0]: 1,
};

class C {
  [x = 0] = 1;
}
```

[プレイグラウンドのリンク](https://biomejs.dev/playground?enabledLinting=false&code=YQAgAD0AIAB7AAoAIAAgAFsAeAAgAD0AIAAwAF0AOgAgADEALAAKAH0ACgAKAGMAbABhAHMAcwAgAEMAIAB7AAoAIAAgACAAIABbAHgAIAA9ACAAMABdACAAPQAgADEACgB9AAoA)

一貫性を保つために、私たちはここで異なる道を選び、括弧を省略することにしました。
代替案としては、オブジェクトまたはクラスの計算されたキーのどの割り当ても括弧で囲むことができます。

### Prettierは必要ない場合でも矢印関数の型パラメータに末尾のカンマを追加します

特定のケースでは、アロー関数の型パラメータリストに末尾のカンマが必要となり、JSX要素と区別する必要があります。
デフォルト型が提供されている場合、この末尾のカンマは必要ありません。
ここで、Prettierの元々の意図をより尊重するために、必要な場合にのみ末尾のカンマを追加するというPrettierから逸脱することにしました。

Input

```tsx title="example.tsx"
<T = unknown>() => {};
```

Diff

```tsx title="example.tsx" del={1} ins={2}
<T = unknown,>() => {};
<T = unknown>() => {};
```


### Prettierは無効な構文をformatします

PrettierのBabelベースのJavaScriptおよびTypeScriptの解析は非常に緩く、[複数のエラーを無視](https://github.com/prettier/prettier/blob/e4a74c05f4502dd4ec70495c3130ff08ab088e05/src/language-js/parse/babel.js#L177-L218) することが許可されています。
Biomeのパーサーは意図的にPrettierのパーサーよりも厳格です。
以下の構文エラーを正確に識別します：

- 関数には重複する修飾子を持つことはできません
- プロパティの修飾子の無効な順序
- 関数宣言には本体を持つことができません
- non-abstract クラスは抽象プロパティを持つことはできません
- オプショナルチェーンに割り当てることはできません
- インターフェースの型パラメータに ``const` 修飾子を設定することはできません
- トップレベルの return
- その他

Prettierでは、これらのエラーは解析エラーとは見なされず、適切なノードでASTが "正しく" 構築されます。
format時にPrettierはこれらのノードを通常通り扱い、それに応じてformatします。

Biomeでは、解析エラーは`Bogus`ノードとして結果に現れ、有効なノード、無効なノード、および/または生の文字のいずれかを含むことがあります。
format時、Biomeは bogusノードを事実上のプレーンテキストとして扱い、formatを試みることが誤って意味を変える可能性があるため、結果のコードにそのまま文字通り出力します。

クラスプロパティに関して、Prettierの現在の解析戦略は修飾子に対してブール値のフィールドを使用し、各種類の修飾子が1つしか存在できないことを意味します（アクセス修飾子は単一の文字列として格納されます）。
出力時に、Prettierはブール値のリストを見て、どの修飾子を再び出力するかを決定します。一方、Biomeは修飾子のリストを保持し、重複を含めて分析が可能になります（これが重複修飾子や順序に関する解析エラーメッセージの原因です）。
不正確なノードを出力する際には、このリストがそのまま維持され、整形されていないテキストの出力はこれらの修飾子が存在し続ける結果となります。

Biomeはこの問題に対処する方法があります。
1つの可能性として、整形時にBogus なノードを解釈し、それらから有効なノードを構築することです。
有効なノードが構築できれば、通常通りそのノードを整形しますが、そうでない場合は現在のように不正確なテキストをそのまま出力します。
しかし、これはやや雑然としており、整形器に意味のない解析ロジックを導入することになります。

別の選択肢としては、純粋に意味論的なエラー（重複修飾子、abstract-class 以外のクラスでの抽象プロパティ）を受け入れる "syntactically-valid bogus node（文法的に不正確なノード）" をパーサーに導入することです。

これにより、通常通りノードを構築しつつ（Prettierの挙動と一致させつつ）、新しい種類の不正確なノードに診断情報を含めて格納します。
整形時には、これら特定の不正確なノードが内部ノードを整形しようと試み、エラーが発生した場合はフォールバックします（既存の `format_or_verbatim` ユーティリティがこれを行います）。
これにより、解析と整形のロジックを分離しつつ、パーサーに複雑さを加えることで、無効な状態を半有効とみなすことが可能になります。

#### クラスプロパティの重複する修飾子

入力

```ts title="example.ts"
// 複数のアクセシビリティ修飾子
class Foo {
  private public a  = 1;
}

// 本体を持つ関数の宣言
declare function foo ( ) {  }

// abstractの不正な使用
class Bar {
  abstract  foo  ;
}

// Readonlyの重複
class Read {
  readonly readonly   x: number;
}
```

差分

```ts title="example.ts" del={3, 8, 13, 19} ins={4, 9, 14, 20}
// 複数のアクセシビリティ修飾子
class Foo {
  private a = 1;
  private public a  = 1;
}

// 本体を持つ関数の宣言
declare function foo() {};
declare function foo ( ) {  }

// abstractの不正な使用
class Bar {
  abstract foo;
  abstract  foo  ;
}

// Readonlyの重複
class Read {
  readonly x: number;
  readonly readonly   x: number;
}


#### オプショナルチェーンへの代入

入力

```js title="example.js"
(a?.b) = c;
```

差分

```js title="example.js" del={1} ins={2}
a?.b = c;
(a?.b) = c;
```

#### インターフェイスの型パラメータに対する誤った修飾子

入力

```ts title="example.js"
interface L<in const T> {}
```

差分

```ts title="example.js" del={1} ins={2}
interface L<const in T> {}
interface L<in const T> {}
```

#### トップレベルのreturn

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

#### 誤った self-increment と self-decrement

入力

```js title="example.js"
(1)++;
```

```js title="example.js" del{1} add={2}
1++;
(1)++;
```

#### 抽象クラスでないクラスでの `abstract` 修飾子の使用

入力

```ts title="example.js"
class C {
  abstract f() : number;
}
```

差分


```ts title="example.js" del{2} add={3}
class C {
  abstract f(): number;
  abstract f() : number;
}
```
