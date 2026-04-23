/* should not generate diagnostics */
import {useEffect} from 'react';

const KEY = "key";

interface ComponentProps {
    key: string;
}

function Component(props: ComponentProps) {
    const { [KEY]: key1 } = props;
    const { key: key2 } = props;
    useEffect(() => {
        console.log(key1)
    }, [key1]);
    useEffect(() => {
        console.log(key2)
    }, [key2]);
    return null;
}
