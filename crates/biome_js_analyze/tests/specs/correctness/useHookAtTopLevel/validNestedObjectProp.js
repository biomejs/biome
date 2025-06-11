/* should not generate diagnostics */
function ReactComponent() {
    const testObj = {
        get print() {
            return "hello"
        }
    }

    const callback = useCallback(() => {}, [])

    return <></>
}
