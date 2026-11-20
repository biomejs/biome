// should generate diagnostics
// Interfaces of one name merge into one type, and a class with them, as in
// TypeScript, so a method of any of the declarations returns its promise.

interface Job {
	run(): Promise<void>;
}

interface Job {
	name: string;
}

declare const job: Job;
job.run();

class Task {
	id = 1;
}

interface Task {
	start(): Promise<void>;
}

new Task().start();

interface Step {
	finish(): Promise<void>;
}

class Step {
	id = 1;
}

declare const step: Step;
step.finish();
