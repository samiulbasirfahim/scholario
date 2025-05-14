import type { Student, StudentRelationship } from '$lib/types/student';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

export const students = $state({
    data: [] as Student[],

    set(list: Student[]) {
        this.data = list;
    },

    get(id: number) {
        return this.data.find((s) => s.id === id);
    },

    add(item: Student) {
        this.data.push(item);
    },

    update(id: number, updated: Student) {
        const index = this.data.findIndex((s) => s.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    },

    remove(id: number) {
        this.data = this.data.filter((s) => s.id !== id);
    },

    async fetch() {
        try {
            const fetched = await invoke<Student[]>('get_students');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching students:', err);
            toast.set({ message: 'Failed to fetch students', type: 'error' });
        }
    }
});

export const studentRelationships = $state({
    data: [] as StudentRelationship[],

    set(list: StudentRelationship[]) {
        this.data = list;
    },

    getByStudent(studentId: number) {
        return this.data.filter((rel) => rel.student_id === studentId);
    },

    getByRelated(relatedId: number) {
        return this.data.filter((rel) => rel.related_id === relatedId);
    },

    add(item: StudentRelationship) {
        this.data.push(item);
    },

    update(id: number, updated: StudentRelationship) {
        const index = this.data.findIndex((rel) => rel.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    },

    remove(id: number) {
        this.data = this.data.filter((rel) => rel.id !== id);
    },

    async fetch() {
        try {
            const fetched = await invoke<StudentRelationship[]>('get_student_relationships');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching student relationships:', err);
            toast.set({ message: 'Failed to fetch relationships', type: 'error' });
        }
    }
});
