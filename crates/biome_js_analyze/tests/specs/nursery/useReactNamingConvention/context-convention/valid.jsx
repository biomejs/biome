/* should not generate diagnostics */
import { createContext } from "react";

const ThemeContext = createContext("");
const Context = createContext("");
obj.nested.ThemeContext = createContext("");

// Class property following the convention.
class Provider {
	ThemeContext = createContext("");
}

// Not the React API.
const theme = notCreateContext("");
