export interface Class {
    id: number;
    name: string;
    level: number;
    admission_fee: number;
    monthly_fee: number;
    readmission_fee: number;
}

export interface Section {
    id: number;
    name: string;
    class_id: number;
}

export interface Subject {
    id: number;
    name: string;
    code: number;
}

export interface ClassSubject {
    id: number;
    class_id: number;
    subject_id: number;
    is_mandatory: boolean;
}

export interface ClassSubjectGroup {
    classId: number;
    subjects: ClassSubject[];
}
