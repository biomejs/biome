interface User {
	id: string;

	updatedAt: Date;
	createdAt: Date;

	passwordHash: string;
	name: string;
	email: string;
}
