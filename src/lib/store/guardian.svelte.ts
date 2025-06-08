import type { Guardian, StudentRelationship } from '$lib/types/guardian';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

export const guardians = $state({
    data: [] as Guardian[],
    set(list: Guardian[]) {
        this.data = list;
    },

    get(id: number) {
        return this.data.find((c) => c.id === id);
    },

    update(id: number, updated: Guardian) {
        const index = this.data.findIndex((c) => c.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    },

    add(item: Guardian) {
        this.data.push(item);
    },
    remove(id: number) {
        this.data = this.data.filter((c) => c.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Guardian[]>('get_guardians');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching classes:', err);
            toast.set({ message: 'Failed to fetch classes', type: 'error' });
        }
    }
});

class StudentRelationships {
    private data: Map<number, StudentRelationship[]> = new Map(); // Map student_id => relationships[]
    private fetchedStudents: Set<number> = new Set();

    private insert(rel: StudentRelationship) {
        if (!this.data.has(rel.student_id)) {
            this.data.set(rel.student_id, []);
        }
        this.data.get(rel.student_id)!.push(rel);
    }

    private fillCache(relationships: StudentRelationship[]) {
        for (const rel of relationships) {
            this.insert(rel);
        }
    }

    async fetch(student_id?: number) {
        try {
            const args: { student_id?: number } = {};
            if (student_id !== undefined) args.student_id = student_id;

            console.log('FETCHING');

            const fetched: StudentRelationship[] = await invoke('get_student_relationships', args);

            this.fillCache(fetched);

            if (student_id !== undefined) {
                this.fetchedStudents.add(student_id);
            }
        } catch (err) {
            console.error('Failed to fetch student relationships:', err);
            toast.set({ message: 'Failed to fetch student relationships', type: 'error' });
        }
    }

    get(student_id: number): StudentRelationship[] {
        return this.data.get(student_id) ?? [];
    }

    add(rel: StudentRelationship) {
        this.insert(rel);
    }

    update(id: number, updated: StudentRelationship) {
        const rels = this.data.get(updated.student_id);
        if (!rels) return;

        const index = rels.findIndex((r) => r.id === id);
        if (index !== -1) {
            rels[index] = updated;
        }
    }

    remove(id: number) {
        for (const [studentId, rels] of this.data.entries()) {
            this.data.set(
                studentId,
                rels.filter((r) => r.id !== id)
            );
        }
    }
}

export const studentRelationships = new StudentRelationships();
