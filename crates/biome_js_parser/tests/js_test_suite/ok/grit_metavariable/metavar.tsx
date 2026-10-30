import $defaultImport from $source;
import { $namedImport, type $namedType } from $source;

$statement;

function foo() {
    $statement;
    const bar = $expression;
}

class Foo {
    $classMember;
}

const { $key: key } = { $key: $value };

function $functionName() {}

type $Type = $OtherType;

interface $Interface {
    $body
}

<$tag $_>$_</$tag>
