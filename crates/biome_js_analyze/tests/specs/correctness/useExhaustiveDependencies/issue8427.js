import { useEffect } from "react";

// Missing diagnostic reported: fe should be included in the deps.
export const Component1 = ({id, fetchEntity}) => {
    const fe = fetchEntity;
    useEffect(() => {
        fe(id);
    }, [id]);
};

// This correctly produces a diagnostic as fetchEntity is not included in the dependency array
export const Component2 = ({id, fetchEntity}) => {
    useEffect(() => {
        fetchEntity(id);
    }, [id])
};

function globalFetchEntity() {}

// This correctly doesn't produce a diagnostic as fe references to a global function
export const Component3 = ({id}) => {
    const fe = globalFetchEntity;
    useEffect(() => {
        fe(id);
    }, [id])
};

// This correctly doesn't produce a diagnostic as fe references to something unknown
export const Component4 = ({id}) => {
    const fe = globalFetchEntityUndefined;
    useEffect(() => {
        fe(id);
    }, [id])
};
