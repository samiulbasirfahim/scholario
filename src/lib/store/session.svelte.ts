import type { Session } from '$lib/types/session';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

class Sessions {
    data = $state<Session[]>([]);
    selected = $state<null | number>(null);

    fetched = false;

    async fetch() {
        if (this.fetched) return;
        try {
            this.data = await invoke<Session[]>('get_sessions');
            this.fetched = true;
            if (this.data.length > 0) {
                const latestSession = this.data.reduce((latest, current) => {
                    return new Date(current.end_date) > new Date(latest.end_date) ? current : latest;
                }, this.data[0]);
                this.selected = latestSession.id;
            } else {
                this.selected = null;
            }
        } catch (err) {
            console.error('Error fetching sessions:', err);
            toast.set({ message: 'Failed to fetch sessions', type: 'error' });
        }
    }

    async edit(session: Session) {
        try {
            const updated = await invoke<Session>('edit_session', {
                ...session
            });
            this.update(session.id, updated);
            toast.set({ message: 'Session updated successfully', type: 'success' });
        } catch (err) {
            console.error('Failed to edit session:', err);
            toast.set({ message: 'Failed to update session', type: 'error' });
        }
    }
    add(session: Session): void {
        this.data.push(session);
        this.selected = session.id;
    }

    select(id: number | null) {
        this.selected = id;
    }

    get selectedSession(): Session | null {
        return this.selected != null ? (this.data.find((s) => s.id === this.selected) ?? null) : null;
    }

    get(id: number) {
        return this.data.find((s) => s.id === id);
    }

    update(id: number, updated: Session): void {
        const index = this.data.findIndex((s) => s.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    }

    remove(id: number): void {
        this.data = this.data.filter((s) => s.id !== id);
    }
}

export const sessions = new Sessions();
