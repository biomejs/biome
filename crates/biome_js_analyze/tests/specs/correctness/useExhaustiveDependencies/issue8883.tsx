import React, { useEffect } from 'react'

type ExampleProps = {
    msg: string;
};

const DestructuredInParams: React.FC<ExampleProps> = ({ msg }) => {
    useEffect(() => console.log(msg), [msg]); // correct behavior
};

const DestructuredInBody1: React.FC<ExampleProps> = props => {
    const { msg } = props;
    useEffect(() => console.log(msg), [msg]); // should NOT trigger useExhaustiveDependencies
};

const DestructuredInBody2: React.FC<ExampleProps> = props => {
    const { msg } = props;
    useEffect(() => console.log(msg), []); // should trigger useExhaustiveDependencies
};
