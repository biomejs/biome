import "very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_";
import "very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long" with { type :   "json"}
import "short" with {

        type :   "json"
}

import "very_long_import_very_long_import_very" with {
    // something good is here
    "type": /****/ "json"
        }

import "very_long_import_very_long_import_very" with {
		// something good is here
		"type": /****/ "json",
		"type2" /****/ : "json",
		/****/
		"type4" /* dangling 1 */: /* danling 2 */ // line
			"json",
		/****/
		"typetypetypetypetypetypetypetypetypetypetype": /****/ "typetypetypetypetypetypetypetypetypetypetypetypetypetype",
		}

