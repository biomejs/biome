/* should generate diagnostics */
<div>
    <div />
    <marquee />
    <blink />
    <marquee>News</marquee>
    <blink>
        <span>Sale</span>
    </blink>
</div>;

export const Ticker = () => <marquee>News</marquee>;

export const Blinker = () => <blink />;
