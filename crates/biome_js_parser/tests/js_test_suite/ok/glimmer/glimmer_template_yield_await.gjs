// Template with yield expression
function* generator() {
    yield <template>Yielded template</template>;
}

// Template with await expression
async function asyncFn() {
    await <template>Awaited template</template>;
}

// Template after typeof
const TypeOf = typeof <template>Type test</template>;

// Template in conditional
const conditional = true ? <template>True</template> : <template>False</template>;

// Template with void
const voidResult = void <template>Void template</template>;

// Template with delete (unusual but valid expression)
const obj = { prop: <template>Prop</template> };
