import type { Session } from '$lib/types/session';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

export const sessions = $state({
    data: [] as Session[],

    set(list: Session[]) {
        this.data = list;
    },

    get(id: number) {
        return this.data.find((s) => s.id === id);
    },

    add(session: Session) {
        this.data.push(session);
    },

    update(id: number, updated: Session) {
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
            const fetched = await invoke<Session[]>('get_sessions');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching sessions:', err);
            toast.set({ message: 'Failed to fetch sessions', type: 'error' });
        }
    }
});
