/* should not generate diagnostics */
// This should be valid - pause() is a function, not page.pause()
function pause() {
    console.log('pausing');
}

pause();

