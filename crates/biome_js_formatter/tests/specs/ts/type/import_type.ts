type ImportType1 = typeof import('source');

type ImportType2 = import('source');

type QualifiedImportType = typeof import('source').Qualified<TypeParams>;

type ActionLogsQ = import("longlonglonglonglonglonglonglongsource").QueryBuilder<"audit.action_logs">;

type LongImportType = typeof import("./long/long/long/long/long/long/long/long/path/long/long/long/long/path").default