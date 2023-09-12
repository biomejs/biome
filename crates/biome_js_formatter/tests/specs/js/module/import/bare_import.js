import "very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_";
import "very_long_import_very_long_import_very_long_import_very_long_import_very_long_import_very_long" assert { type :   "json"}
import "short" assert {

        type :   "json"
}

import "very_long_import_very_long_import_very" assert {
    // something good is here
    "type": /****/ "json"
        }

import "very_long_import_very_long_import_very" assert {
		// something good is here
		"type": /****/ "json",
		"type2" /****/ : "json",
		/****/
		"type4" /* dangling 1 */: /* danling 2 */ // line
			"json",
		/****/
		"typetypetypetypetypetypetypetypetypetypetype": /****/ "typetypetypetypetypetypetypetypetypetypetypetypetypetype",
		}

