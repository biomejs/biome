/* should generate diagnostics */
for (var i = 0; i < 10; i++) {
    const process = (event: Event) => {
        return i + event.timeStamp;
    };

    queue.push(process);
}
