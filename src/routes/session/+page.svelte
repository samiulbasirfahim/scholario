<script lang="ts">
	import Toast from '$lib/components/global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Session } from '$lib/types/session';
	import { onMount } from 'svelte';

	function formatDate(date: Date): string {
		return date.toISOString().split('T')[0];
	}

	const today = new Date();
	const oneYearLater = new Date(today);
	oneYearLater.setFullYear(today.getFullYear() + 1);

	let form = $state({
		id: null as number | null,
		name: '',
		start_date: formatDate(today),
		end_date: formatDate(oneYearLater)
	});

	let selectedSession = $derived(sessions.get(sessions.selected as number));
	let isEditing = $state(false);

	function formatMoney(amount: number): string {
		return (
			'৳' +
			(amount / 100).toLocaleString('en-BD', {
				style: 'decimal',
				minimumFractionDigits: 2
			})
		);
	}

	function resetForm() {
		form = {
			id: null,
			name: '',
			start_date: formatDate(new Date()),
			end_date: formatDate(new Date(new Date().setFullYear(new Date().getFullYear() + 1)))
		};
		isEditing = false;
	}

	function addOrUpdateSession() {
		if (form.name.trim() && form.start_date && form.end_date) {
			if (isEditing && form.id != null) {
				sessions.edit({
					id: form.id,
					name: form.name,
					end_date: form.end_date,
					start_date: form.start_date
				});
				resetForm();
			} else {
				invoke('create_session', { ...form })
					.then((d) => {
						sessions.add(d as Session);
						toast.set({ message: 'Session created successfully', type: 'success' });
						resetForm();
					})
					.catch((e) => {
						toast.set({ message: 'Failed to create session', type: 'error' });
						console.log(e);
					});
			}
		} else {
			toast.set({ message: 'Please fill in all required fields', type: 'error' });
		}
	}

	function editSession(session: Session) {
		form = { ...session };
		isEditing = true;
	}

	function removeSession(id: number) {
		invoke('delete_session', { id })
			.then(() => {
				sessions.remove(id);
				toast.set({ message: 'Session deleted successfully', type: 'success' });
				if (sessions.selected === id) {
					sessions.selected = null;
				}
			})
			.catch((e) => {
				console.error('Failed to delete session:', e);
				toast.set({ message: 'Failed to delete session', type: 'error' });
			});
	}

	$effect(() => {
		console.log($state.snapshot(sessions.data));
	});
</script>

<h2 class="text-primary mb-3 text-xl font-bold">Manage Sessions</h2>
<div class="flex gap-2">
	<div class="w-1/2">
		<form class="bg-base-100 border-base-300 space-y-3 rounded border p-4">
			<h2 class="text-primary mb-3 text-lg font-semibold">
				{isEditing ? 'Edit Session' : 'Create New Session'}
			</h2>

			<input
				bind:value={form.name}
				type="text"
				placeholder="Session name"
				class="input input-bordered w-full"
			/>
			<div class="flex gap-3">
				<input bind:value={form.start_date} type="date" class="input input-bordered w-full" />
				<input bind:value={form.end_date} type="date" class="input input-bordered w-full" />
			</div>
			<div class="flex gap-3">
				<button type="submit" onclick={addOrUpdateSession} class="btn btn-primary w-full">
					{isEditing ? 'Update' : 'Add'}
				</button>
				{#if isEditing}
					<button type="button" onclick={resetForm} class="btn btn-ghost w-full">Cancel</button>
				{/if}
			</div>
		</form>

		<div class="bg-base-100 border-base-300 mt-4 space-y-3 rounded border p-4">
			{#if sessions.data.length > 0}
				<div class="overflow-x-auto">
					<table class="table">
						<thead>
							<tr>
								<th>#</th>
								<th>Name</th>
								<th>Start date</th>
								<th>End date</th>
							</tr>
						</thead>
						<tbody>
							{#each sessions.data as session, i (session.id)}
								<tr
									class="{session.id === sessions.selected
										? 'bg-primary text-primary-content'
										: ''} cursor-pointer"
									onclick={() => {
										sessions.select(session.id);
									}}
								>
									<th>{i + 1}</th>
									<td>{session.name}</td>
									<td>{session.start_date}</td>
									<td>{session.end_date}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{:else}
				<p class="text-secondary text-sm">No sessions added yet.</p>
			{/if}
		</div>
	</div>

	<div class="w-1/2">
		<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
			<h2 class="text-primary mb-3 text-xl font-bold">Session Details</h2>
			{#if sessions.selected}
				<div class="space-y-3 text-sm">
					<div>
						<p class="text-secondary">Name</p>
						<p class="font-medium">{selectedSession?.name}</p>
					</div>
					<div>
						<p class="text-secondary">Start</p>
						<p class="font-medium">{selectedSession?.start_date}</p>
					</div>
					<div>
						<p class="text-secondary">End</p>
						<p class="font-medium">{selectedSession?.end_date}</p>
					</div>

					<div class="flex flex-wrap gap-3 pt-2">
						<button
							class="btn btn-info w-full sm:w-auto"
							onclick={() => editSession(selectedSession as Session)}>Edit</button
						>
						<button
							class="btn btn-error w-full sm:w-auto"
							onclick={() => removeSession(selectedSession?.id as number)}>Delete</button
						>
					</div>

					<hr class="border-base-300 my-3" />

					<div class="grid grid-cols-2 gap-3 text-sm">
						<div>
							<p class="text-secondary">Total Students</p>
							<p class="font-medium">[DUMMY]</p>
						</div>
						<div>
							<p class="text-secondary">Total Classes</p>
							<p class="font-medium">[DUMMY]</p>
						</div>
						<div>
							<p class="text-secondary">Incoming</p>
							<p class="text-success font-medium">{formatMoney(10000)}</p>
						</div>
						<div>
							<p class="text-secondary">Outgoing</p>
							<p class="text-error font-medium">{formatMoney(10000)}</p>
						</div>
						<div class="col-span-2">
							<p class="text-secondary">Unpaid</p>
							<p class="text-info font-medium">{formatMoney(10000)}</p>
						</div>
					</div>
				</div>
			{:else}
				<p class="text-secondary text-sm">No session is selected.</p>
			{/if}
		</div>
	</div>
</div>

<Toast />
