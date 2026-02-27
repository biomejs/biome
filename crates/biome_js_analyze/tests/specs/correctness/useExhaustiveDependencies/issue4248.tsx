import React from "react";

type ExampleProps = {
    getData: () => string;
};

const ExampleComponent: React.FC<ExampleProps> = ({ getData }) => {
    // Incorrect diagnostic reported: getData should be removed from deps
    const data = React.useMemo(getData, [getData]);
    return <span>{data}</span>;
};
