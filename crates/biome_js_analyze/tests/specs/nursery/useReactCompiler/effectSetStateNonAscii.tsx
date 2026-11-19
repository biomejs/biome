// should generate diagnostics

import { useEffect, useState } from "react";

export function useOnlineStatus({ paused }: { paused: boolean }) {
    const [isOnline, setIsOnline] = useState(true);

    // 非ASCII文字を含むコメントなので、これ以降はUTF-16のオフセットとUTF-8のバイト位置がずれる。
    useEffect(() => {
        setIsOnline(true);
    }, [paused]);

    return isOnline;
}
