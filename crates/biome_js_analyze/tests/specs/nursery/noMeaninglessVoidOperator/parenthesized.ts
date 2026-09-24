/* should generate diagnostics */
declare function log(): void;
void (log());
void /* before parentheses */ (log() as void);
void // before parentheses
    (log() as void);
void (/* inside parentheses */ log() as void);
void ((log() as void));
void (log() as void) // after parentheses
;
function returns() {
    return void // after return
        (log());
}
function throws() {
    throw void // after throw
        (log());
}
