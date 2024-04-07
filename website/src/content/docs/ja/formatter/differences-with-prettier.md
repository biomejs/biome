---
title: Prettier との違い
description: Prettier との違いを深く解説
---

Prettier との間にいくつかの相違点が存在します。

## Prettierは有効なJavaScript識別子である一部のオブジェクトプロパティの引用符を外しません

PrettierとBiomeは、有効なJavaScript識別子であるオブジェクトおよびクラスプロパティの引用符を外します。
Prettierは、特に[ES5において有効な識別子のみ引用符を外します](https://github.com/prettier/prettier/blob/a5d502513e5de4819a41fd90b9be7247146effc7/src/language-js/utils/index.js#L646)。

ES2015が広まっている現在のエコシステムにおいて、これは古い制限です。
そのため、私たちはES2015以降の全ての有効なJavaScript識別子の引用符を外すことにしました。

プロジェクトが使用するECMAScriptバージョンを設定するための設定を導入することは可能な解決策かもしれません。
そのバージョンに基づいて引用符を外す動作を調整できます。
ECMAScriptのバージョンをES5に設定することで、Prettierの挙動に一致させることができます。

入力

```js title="example.js"
const obj = {
 'a': true,
 b: true,
 "𐊧": true,
}
```

差分

```js title="exmaple.js" del={4} ins={5}
const obj = {
  a: true,
  b: true,
  "𐊧": true,
  𐊧: true,
};
```


## Prettierは計算プロパティでの代入で一貫性のない挙動を示します

PrettierとBiomeは、特に条件文などで代入式を括弧で囲みます。
これにより、Biomeは比較式であるべきコードを識別できます。

Prettierは、オブジェクトの計算プロパティでの代入では括弧を追加する一方で、クラスの計算プロパティではそれを行いません。以下の例で示されます：

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

一貫性を保つために、私たちは Prettier のformatに合わせることなく括弧を省略することにしました。
代替案としては、オブジェクトまたはクラスの計算プロパティでの代入を常に括弧で囲むことができます。

## Prettierは必要ない場合でもアロー関数の型パラメータに末尾のカンマを追加します

特定のケースでは、JSX要素と区別するために、アロー関数の型パラメータリストに末尾のカンマが必要となります。
型パラメータにデフォルト値が提供されている場合、末尾のカンマは必要ありません。
ここでは、必要な場合にのみ末尾のカンマを追加するというPrettierの元々の意図を尊重するために、現在のformatの差分を受け入れました。

入力

```tsx title="example.tsx"
<T = unknown>() => {};
```

差分

```tsx title="example.tsx" del={1} ins={2}
<T = unknown,>() => {};
<T = unknown>() => {};
```

## Prettier は、括弧で囲まれた non-null アサーションを含むオプショナルチェーンに対して一貫性のない動作をします

_TypeScript_ では、non-null アサーション演算子 `!` を使用して、値が null でないことをアサートできます。
オプショナルチェーンに適用される場合、アサーションは括弧の存在に関係なくチェーン全体に適用されます。つまり、`(a.?.b)!` と `a.?.b!` は同じ結果になるはずです。

Prettier は、先のコード例をどちらも適切にformatされているとみなします。つまり、括弧の有無は維持され、コードを正規化しようとしません。

さらに、Prettier は、括弧がnon-null アサーション演算子を囲んでいる場合でもかっこを削除しません。代わりに、演算子を括弧の外に移動します。

入力

```ts title="example.ts"
a.?.b!
(a.?.b)!
(a.?.b!)
```

差分

```ts title="example.ts" del={2, 4} ins={3, 5}
a.?.b!
(a.?.b)!
a.?.b!
(a.?.b)!
a.?.b!
```


## Prettierは無効な構文をformatします

JavaScriptおよびTypeScriptのためにPrettierで利用されているBabel parserの解析は厳密なものではなく、[いくつかの構文エラーを無視](https://github.com/prettier/prettier/blob/e4a74c05f4502dd4ec70495c3130ff08ab088e05/src/language-js/parse/babel.js#L177-L218) することがあります。
Biomeのparserは、Prettierのparserよりも厳密に構文を解析します。
たとえば、以下の構文エラーを正確に識別します：

- 関数には重複する修飾子を持つことはできません
- プロパティの修飾子を無効な順序で指定することはできません
- 関数宣言では関数の処理を定義することができません
- 抽象クラス以外で抽象プロパティを持つことはできません
- オプショナルチェーンに代入することはできません
- インターフェースの型パラメータに `const` 修飾子を設定することはできません
- トップレベルの return

Prettierでは、これらのエラーを構文エラーとして扱わず、適切なノードでASTが "正しく" 構築されます。
そして、これらのノードを通常通り扱い、formatします。

Biomeでは、構文エラーは`Bogus`ノードとして扱います。`Bogus`ノードは、有効なノード、無効なノード、生の文字などを含みます。
format時、Biomeは bogusノードを事実上のプレーンテキストとして扱い、そのまま出力します。これは、formatを試みることでコードの意図を変更してしまう恐れがあるためです。

Prettierのparserは、クラスプロパティの修飾子に対してブール値のフィールドを使用します。これは、各種類の修飾子が1つしか存在できないことを意味します。（アクセス修飾子は単一の文字列として格納されます）。
formatの時には、ブール値のリストを見て、どの修飾子を再び出力するかを決定します。一方、Biomeは修飾子のリストを保持し、重複を含めて解析が可能になります（これが重複修飾子や順序に関する解析エラーを報告できる理由です）。
不正な修飾子を含むbogusノードをformatする際は、修飾子のリストはそのまま維持され、不正なテキストをそのまま出力します。

Biomeでは、この問題に対処するいくつかの方法があります。
その1つとしては、format時にBogus なノードを解釈し、有効なノードを構築する方法があります。
有効なノードが構築できれば通常通りそのノードをformatし、そうでない場合は現在のように不正なテキストをそのまま出力します。
しかし、これを実装するのは少し大変で、formatterに意味のないロジックを導入することになります。

別の方法としては、意味論的なエラー（重複修飾子、抽象クラス以外での抽象プロパティの使用）を受け入れる "syntactically-valid bogus node（文法的に不正確なノード）" をparserに導入する方法もあります。

これにより、通常通りノードを構築しつつ（Prettierの挙動と一致させつつ）、新しい種類のbogusノードに構文解析の情報を含めて格納します。
format時には、特定のbogusノードでは内部ノードをformatしようと試み、エラーが発生した場合はフォールバックします（既存の `format_or_verbatim` ユーティリティがこれを行っています）。
これにより、parserとformatterのロジックを分離しつつ、無効な状態を半有効とみなすような複雑なロジックをparserに実装することが可能になります。

### クラスプロパティの重複する修飾子

入力

```ts title="example.ts"
// 複数のアクセシビリティ修飾子
class Foo {
  private public a  = 1;
}

// 処理を持つ関数の宣言
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

// 処理を持つ関数の宣言
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


### オプショナルチェーンへの代入

入力

```js title="example.js"
(a?.b) = c;
```

差分

```js title="example.js" del={1} ins={2}
a?.b = c;
(a?.b) = c;
```

### インターフェイスの型パラメータに対する誤った修飾子

入力

```ts title="example.js"
interface L<in const T> {}
```

差分

```ts title="example.js" del={1} ins={2}
interface L<const in T> {}
interface L<in const T> {}
```

### トップレベルのreturn

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

### 誤った self-increment と self-decrement

入力

```js title="example.js"
(1)++;
```

```js title="example.js" del{1} add={2}
1++;
(1)++;
```

### 抽象クラスでないクラスでの `abstract` 修飾子の使用

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
