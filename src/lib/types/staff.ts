
export interface Staff {
	id: number;
	name: string;
	phone: string;
	address: string;
	salary: number;
	hire_date: string; // ISO date string (YYYY-MM-DD)
	photo: string | null;
	is_teacher: boolean;
	role: string;
	qualification: string;
}
