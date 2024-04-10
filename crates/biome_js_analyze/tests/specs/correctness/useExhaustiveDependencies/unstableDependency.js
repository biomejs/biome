import { useState } from "react";

const Component = () => {
    const [things, setThings] = useState(undefined)

    const fetchThings = async () => {
        const t = await fetchSomething()
        if (t) {
            setThings('done')
        }
    }

    useEffect(() => {
        const fetchData = async () => {
            await fetchThings()
        }

        fetchData();
    }, [fetchThings])

    if (!things) {
        return <div>Loading...</div>
    }

    return (
        <div>Loaded some things</div>
    )
}
