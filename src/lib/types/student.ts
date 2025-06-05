export interface Student {
    id: number;
    name: string;
    class_id: number;
    section_id: number;
    session_id: number;
    dob: string; // ISO date string
    gender: string;
    religion: string;
    address: string;
    phone: string | null;
    admission_date: string; // ISO date string
    is_resident: boolean;
    roll: number;
    photo: string | null;
    health_notes: string | null;
    general_notes: string | null;
}
