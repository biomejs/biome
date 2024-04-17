import { useEffect, useState } from "react";

const Component = () => {
    const [things, setThings] = useState(undefined);

    const fetchThings = async () => {
        const t = await fetchSomething();
        if (t) {
            setThings('done');
        }
    }

    function fetchMoreThings() {
        return fetchThings();
    }

    const mapping = {
        something: things
    };

    useEffect(() => {
        fetchThings();
        const fetchData = async () => {
            await fetchMoreThings();
        }

        fetchData().then((res) => {
            return mapping.something[res.body] || res;
        });
    }, [fetchThings, fetchMoreThings, mapping.something]);

    if (!things) {
        return <div>Loading...</div>;
    }

    return (
        <div>Loaded some things</div>
    );
}
