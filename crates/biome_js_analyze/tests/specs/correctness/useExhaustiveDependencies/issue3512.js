import { useState, useEffect } from 'react'

// Problem
function StateArrayDestructure() {
    const [[x, y], setXY] = useState([1, 2]);
    // Incorrect diagnostic reported: says "y" should be removed
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}

// Problem
function StateArrayDestructureWithMissing() {
    const [[x, y], setXY] = useState([1, 2]);
    // Missing diagnostic reported: should say "y" should be added
    useEffect(() => {
        console.log(x, y);
    }, [x]);

    return null
}

// Ok
function ArrayDestructure() {
    const [x, y] = [Math.random(), Math.random()];
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}

// Ok
function StateObjectDestructure() {
    const [{ x, y }, setXY] = useState({ x: 1, y: 2 });
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}

// Ok
function ObjectDestructure() {
    const {x, y} = { x: Math.random(), y: Math.random() };
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}

// Ok
function StateIndependentVariables() {
    const [x, setX] = useState(1);
    const [y, setY] = useState(2);
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}

// Ok
function IndependentVariables() {
    const x = Math.random();
    const y = Math.random();
    useEffect(() => {
        console.log(x, y)
    }, [x, y]);

    return null
}
