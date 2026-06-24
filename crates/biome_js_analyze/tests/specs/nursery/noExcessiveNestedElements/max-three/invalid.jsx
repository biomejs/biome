// should generate diagnostics
function Component() {
    return (
        <div>
            <div>
                <div>
                    <div>
                        <span>Too deeply nested with maxDepth: 3!</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
