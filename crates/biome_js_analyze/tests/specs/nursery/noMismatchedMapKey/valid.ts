/* should not generate diagnostics */
if (map.has(key as string)) { map.get(key); }
if ((map as Map<string, number>).has(key)) { map.get(key!); }
if (map.has(key satisfies string)) { map.get(<string>key); }
if (map.has(user?.id)) { map.get(user!.id); }
