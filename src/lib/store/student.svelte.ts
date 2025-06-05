import type { Student } from '$lib/types/student';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';
import { sessions } from './session.svelte';

class Students {
    data: Map<number, Map<number, Map<number, Student[]>>> = new Map();

    fetchedKeys: Set<string> = new Set();

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

    private async fetch(session_id: number, class_id?: number, section_id?: number) {
        if (sessions.selected === null) return;
        try {
            const args: { sessionId: number; classId?: number; sectionId?: number } = {
                sessionId: session_id
            };
            if (class_id !== undefined) args.classId = class_id;
            if (section_id !== undefined) args.sectionId = section_id;

            const fetchedStudents: Student[] = await invoke('get_students', args);
            this.fillCache(fetchedStudents);
        } catch (err) {
            console.error('Failed to fetch students:', err);
            toast.set({ message: 'Failed to fetch students', type: 'error' });
        }
    }

    add(student: Student): void {
        this.insert(student);
    }

    getById(id: number): Student | undefined {
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
        const sectionKey = this.makeKey(session_id, class_id, section_id);
        const sessionKey = this.makeKey(session_id);
        const classKey = this.makeKey(session_id, class_id);

        if (
            !(
                this.fetchedKeys.has(sessionKey) ||
                (class_id !== undefined && this.fetchedKeys.has(classKey)) ||
                this.fetchedKeys.has(sectionKey)
            )
        ) {
            await this.fetch(session_id, class_id, section_id);
            this.fetchedKeys.add(sectionKey);
        }

        const classMap = this.data.get(session_id);

        if (!classMap) {
            // No data at all for this session
            return [];
        }

        if (class_id === undefined) {
            // Return all students in the session (flatten all classes & sections)
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
            // No data for this class
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
}

export const students = new Students();
