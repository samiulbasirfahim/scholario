import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

import type { Section, Subject, Class, ClassSubject } from '$lib/types/class';

export const classes = $state({
    data: {} as Record<number, Class[]>,

    set(session_id: number, list: Class[]) {
        this.data[session_id] = list;
    },

    get_by_session(session_id: number) {
        return this.data[session_id] ?? [];
    },

    get(session_id: number, id: number) {
        return this.data[session_id]?.find((c) => c.id === id);
    },

    update(session_id: number, id: number, updated: Class) {
        const list = this.data[session_id];
        if (!list) return;
        const index = list.findIndex((c) => c.id === id);
        if (index !== -1) {
            list[index] = updated;
        }
    },

    add(session_id: number, item: Class) {
        if (!this.data[session_id]) {
            this.data[session_id] = [];
        }
        this.data[session_id].push(item);
    },

    remove(session_id: number, id: number) {
        if (!this.data[session_id]) return;
        this.data[session_id] = this.data[session_id].filter((c) => c.id !== id);
    },

    async fetch(session_id: number) {
        try {
            const fetched = await invoke<Class[]>('get_classes', { session_id });
            console.log(fetched);
            this.set(session_id, fetched);
        } catch (err) {
            console.error('Error fetching classes for session:', err);
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

    get(id: number) {
        return this.data.find((d) => d.id === id);
    },
    remove(id: number) {
        this.data = this.data.filter((s) => s.id !== id);
    },

    update(item: Subject) {
        this.data = this.data.map((s) => (s.id === item.id ? item : s));
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

    remove_from_all(id: number) {
        for (const classId in this.data) {
            this.data[classId] = this.data[classId].filter((s) => s.id !== id);
        }
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
    },

    async fetch_all() {
        classes.data.forEach(async (c) => {
            try {
                const fetched = await invoke<ClassSubject[]>('get_class_subjects_by_class', {
                    class_id: c.id
                });
                console.log(fetched);
                this.set(c.id, fetched);
            } catch (err) {
                console.error(`Error fetching class subjects for class ${c.name}:`, err);
                toast.set({ message: `Failed to fetch subjects for class ${c.name}`, type: 'error' });
            }
        });
    }
});
