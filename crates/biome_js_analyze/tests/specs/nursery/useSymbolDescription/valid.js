/* should not generate diagnostics */
Symbol('foo')
Symbol(1)
const symbolName = 'foo'
Symbol(symbolName)

NotASymbol()

{
    // Redeclare global Symbol
    const Symbol = (name) => name
    Symbol()
}