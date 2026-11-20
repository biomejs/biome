// should generate diagnostics
// Interfaces of one name merge into one type, and a class with them, as in
// TypeScript, so a condition on a member of any of the declarations sees the
// member's type.

interface Config {
	handler: () => void;
}

interface Config {
	name: string;
}

function first(config: Config) {
	if (config.handler) {
		return 1;
	}
}

interface Job {
	run: () => void;
}

class Job {
	id = 1;
}

function classMember(job: Job) {
	if (job.run) {
		return 1;
	}
}
