/* should generate diagnostics */
function localSignature(value: string): string;
export function localSignature(value: string) { return value; }

export function exportedSignature(value: string): string;
function exportedSignature(value: string) { return value; }

function outerSignature(value: string): string;
function outer() {
    function outerSignature(value: string) { return value; }
    function innerSignature(value: string): string;
}
function innerSignature(value: string) { return value; }

switch (value) {
    case 0:
        function switchSignature(value: string): string;
}
function switchSignature(value: string) { return value; }
switch (value) {
    case 0:
        function otherSwitch(value: string) { return value; }
}
