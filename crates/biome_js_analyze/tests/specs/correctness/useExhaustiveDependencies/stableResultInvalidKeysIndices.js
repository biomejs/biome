import { useCallback } from "react";
import { useDispatch } from "react-redux";

function MyComponent25() {
    const dispatch = useDispatch();
    const doAction = useCallback(() => dispatch(someAction()), []);
}
