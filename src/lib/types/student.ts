export interface Student {
    id: number;
    name: string;
    class_id: number;
    section_id: number;
    dob: string; // or Date if you parse it
    gender: string;
    religion: string;
    address: string;
    phone: string;
    admission_date: string;
    is_resident: boolean;
    photo?: string | null;
}

export interface StudentRelationship {
    id: number;
    student_id: number;
    related_id: number;
    relationship?: string | null;
}
