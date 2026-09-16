/* should not generate diagnostics */

export interface Usage {
    range: { startDate: string };
}

export interface LogEntry {
    id: string;
    createdAt: Date;
}

export declare function getUsage(userId: string): Promise<Usage | null>;
export declare function getLogs(): Promise<LogEntry[]>;
