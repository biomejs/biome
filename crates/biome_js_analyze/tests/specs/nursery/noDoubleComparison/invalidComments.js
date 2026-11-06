/* should generate diagnostics */
x === y || /* dropped */ x < y;
x === y || x /* dropped */ < y;
x === y /* dropped */ || x < y;
