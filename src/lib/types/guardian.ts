export interface Guardian {
    id: number;
    name: string;
    phone: string;
    photo: string;
    address: string;
}

export interface StudentRelationship {
    id: number;
    student_id: number;
    related_id: number;
    relationship?: string | null;
}
