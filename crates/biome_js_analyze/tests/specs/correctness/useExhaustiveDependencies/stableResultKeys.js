import { useCallback } from "react";
import { useDispatch } from "react-redux";

function useCustomHook() {
    const dispatch = useDispatch();
    return { dispatch }
}

function MyComponent27() {
    const { dispatch } = useCustomHook();
    const doAction = useCallback(() => dispatch(someAction()), []);
}
