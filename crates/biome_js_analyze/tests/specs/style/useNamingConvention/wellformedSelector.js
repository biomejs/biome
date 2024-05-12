{
	"$schema": "../../../../../../packages/@biomejs/biome/configuration_schema.json",
	"linter": {
		"rules": {
			"style": {
				"useNamingConvention": {
					"level": "error",
					"options": {
						"custom": [
							{
								"selector": {
									"kind": "classProperty",
									"modifiers": ["protected", "static", "readonly"]
								},
								"match": ".*"
							}, {
								"selector": {
									"kind": "typeProperty",
									"modifiers": ["readonly"]
								},
								"match": ".*"
							}, {
								"selector": {
									"kind": "variableLike",
									"scope": "global"
								},
								"match": ".*"
							}, {
								"selector": {
									"kind": "typeLike",
									"scope": "global"
								},
								"match": ".*"
							}
						]
					}
				}
			}
		}
	}
}
