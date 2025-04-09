import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

import type { Section, Subject, Class, ClassSubject } from '$lib/types/class';

export const classes = $state({
    data: [] as Class[],
    set(list: Class[]) {
        this.data = list;
    },

    get(id: number) {
        return this.data.find((c) => c.id === id);
    },

    update(id: number, updated: Class) {
        const index = this.data.findIndex((c) => c.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    },

    add(item: Class) {
        this.data.push(item);
    },
    remove(id: number) {
        this.data = this.data.filter((c) => c.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Class[]>('get_classes');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching classes:', err);
            toast.set({ message: 'Failed to fetch classes', type: 'error' });
        }
    }
});

export const sections = $state({
    data: [] as Section[],
    set(list: Section[]) {
        this.data = list;
    },

    get_by_class(class_id: number) {
        return this.data.filter((s) => s.class_id === class_id);
    },

    get(id: number) {
        return this.data.find((s) => s.id === id);
    },

    add(item: Section) {
        this.data.push(item);
    },
    remove(id: number) {
        this.data = this.data.filter((s) => s.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Section[]>('get_sections');
            console.log(fetched);
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching sections:', err);
            toast.set({ message: 'Failed to fetch sections', type: 'error' });
        }
    }
});

export const subjects = $state({
    data: [] as Subject[],
    set(list: Subject[]) {
        this.data = list;
    },
    add(item: Subject) {
        this.data.push(item);
    },
    remove(id: number) {
        this.data = this.data.filter((s) => s.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Subject[]>('get_subjects');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching subjects:', err);
            toast.set({ message: 'Failed to fetch subjects', type: 'error' });
        }
    }
});

export const classSubjects = $state({
    data: {} as Record<number, ClassSubject[]>,

    set(class_id: number, items: ClassSubject[]) {
        this.data[class_id] = items;
    },

    add(class_id: number, item: ClassSubject) {
        if (!this.data[class_id]) {
            this.data[class_id] = [];
        }
        this.data[class_id].push(item);
    },

    remove(classId: number, id: number) {
        if (!this.data[classId]) return;
        this.data[classId] = this.data[classId].filter((s) => s.id !== id);
    },

    get(classId: number): ClassSubject[] {
        return this.data[classId] ?? [];
    },

    async fetch(class_id: number) {
        if (this.data[class_id]) return; // already cached

        try {
            const fetched = await invoke<ClassSubject[]>('get_class_subjects_by_class', { class_id });
            console.log(fetched);
            this.set(class_id, fetched);
        } catch (err) {
            console.error(`Error fetching class subjects for class ${class_id}:`, err);
            toast.set({ message: `Failed to fetch subjects for class ${class_id}`, type: 'error' });
        }
    }
});
