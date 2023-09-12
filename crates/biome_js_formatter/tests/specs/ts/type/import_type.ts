type ImportType1 = typeof import('source');

type ImportType2 = import('source');

type QualifiedImportType = typeof import('source').Qualified<TypeParams>;

