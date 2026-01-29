import { useEffect } from 'react';

// Test: Arrow function with parameter without parentheses, using props directly
const TestDirect = props => {
  useEffect(() => console.log(props.msg), [props.msg]);
};

// Test: Arrow function with parameter without parentheses, destructuring in body
const TestDestructure = props => {
  const { msg } = props;
  useEffect(() => console.log(msg), [msg]);
};
