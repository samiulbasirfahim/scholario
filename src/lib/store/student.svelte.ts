import type { Student } from '$lib/types/student';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';
import { sessions } from './session.svelte';

class Students {
    data: Map<number, Map<number, Map<number, Student[]>>> = new Map();
    selected: number | null = null;
    fetchedKeys = new Set();
    reactiveCounter = $state(0);

    private makeKey(session_id: number, class_id?: number, section_id?: number): string {
        return `${session_id}-${class_id ?? 'all'}-${section_id ?? 'all'}`;
    }

    private insert(student: Student) {
        if (!this.data.has(student.session_id)) {
            this.data.set(student.session_id, new Map());
        }
        const classMap = this.data.get(student.session_id)!;

        if (!classMap.has(student.class_id)) {
            classMap.set(student.class_id, new Map());
        }
        const sectionMap = classMap.get(student.class_id)!;

        if (!sectionMap.has(student.section_id)) {
            sectionMap.set(student.section_id, []);
        }

        sectionMap.get(student.section_id)!.push(student);
    }

    private fillCache(students: Student[]) {
        for (const student of students) {
            this.insert(student);
        }
    }

    async fetch(session_id: number, class_id?: number, section_id?: number) {
        const sessionKey = this.makeKey(session_id);
        const classKey = this.makeKey(session_id, class_id);
        const sectionKey = this.makeKey(session_id, class_id, section_id);

        const alreadyFetched =
            this.fetchedKeys.has(sessionKey) ||
            (class_id !== undefined && this.fetchedKeys.has(classKey)) ||
            this.fetchedKeys.has(sectionKey);

        if (alreadyFetched) return;

        if (sessions.selected === null) return;

        try {
            const args: { session_id: number; class_id?: number; section_id?: number } = {
                session_id
            };
            if (class_id !== undefined) args.class_id = class_id;
            if (section_id !== undefined) args.section_id = section_id;

            const fetchedStudents: Student[] = await invoke('get_students', args);
            this.fillCache(fetchedStudents);

            toast.warning('FETCHING STUDENTS');

            let newKey: string;
            if (section_id !== undefined) {
                newKey = sectionKey;
            } else if (class_id !== undefined) {
                newKey = classKey;
            } else {
                newKey = sessionKey;
            }

            this.fetchedKeys = new Set(this.fetchedKeys).add(newKey);
            this.reactiveCounter++;
        } catch (err) {
            console.error('Failed to fetch students:', err);
            toast.set({ message: 'Failed to fetch students', type: 'error' });
        }
    }

    add(student: Student): void {
        this.insert(student);
        this.reactiveCounter++;
    }

    getById(id: number): Student | undefined {
        console.log('Reactive counter', this.reactiveCounter);
        const classMap = this.data.get(sessions.selected as number);
        if (!classMap) return undefined;

        for (const sectionMap of classMap.values()) {
            for (const students of sectionMap.values()) {
                const found = students.find((s) => s.id === id);
                if (found) return found;
            }
        }

        return undefined;
    }

    async get(session_id: number, class_id?: number, section_id?: number): Promise<Student[]> {
        console.log('Reactive counter', this.reactiveCounter);
        await this.fetch(session_id, class_id, section_id);

        const classMap = this.data.get(session_id);

        if (!classMap) {
            return [];
        }

        if (class_id === undefined) {
            let allStudents: Student[] = [];
            for (const sectionMap of classMap.values()) {
                for (const students of sectionMap.values()) {
                    allStudents = allStudents.concat(students);
                }
            }
            return allStudents;
        }

        const sectionMap = classMap.get(class_id);
        if (!sectionMap) {
            return [];
        }

        if (section_id === undefined) {
            let allStudents: Student[] = [];
            for (const students of sectionMap.values()) {
                allStudents = allStudents.concat(students);
            }
            return allStudents;
        }

        const students = sectionMap.get(section_id);
        if (!students) {
            return [];
        }

        return students;
    }

    update(updatedStudent: Student): void {
        const { session_id, class_id, section_id, id } = updatedStudent;
        const classMap = this.data.get(session_id);
        if (!classMap) return;

        const sectionMap = classMap.get(class_id);
        if (!sectionMap) return;

        const studentList = sectionMap.get(section_id);
        if (!studentList) return;

        const index = studentList.findIndex((s) => s.id === id);
        if (index !== -1) {
            studentList[index] = updatedStudent;
        }
        this.reactiveCounter++;
    }

    remove(studentId: number): void {
        const student = this.getById(studentId);
        if (!student) return;

        const { session_id, class_id, section_id, id, roll } = student;

        const classMap = this.data.get(session_id);
        if (!classMap) return;

        const sectionMap = classMap.get(class_id);
        if (!sectionMap) return;

        const studentList = sectionMap.get(section_id);
        if (!studentList) return;

        const index = studentList.findIndex((s) => s.id === id);
        if (index === -1) return;

        studentList.splice(index, 1);

        for (const s of studentList) {
            if (s.roll > roll) {
                console.log(s);
                s.roll -= 1;
            }
        }

        this.reactiveCounter--;
    }
}

export const students = new Students();
