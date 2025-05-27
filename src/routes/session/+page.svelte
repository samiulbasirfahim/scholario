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
					start_date: form.start_date,
					end_date: form.end_date
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
		sessions.remove(id);
		invoke('delete_session', { id }).catch((e) => {
			console.error('Failed to delete session:', e);
			toast.set({ message: 'Failed to delete session', type: 'error' });
		});
	}

	onMount(() => {
		sessions.fetch();
	});
</script>

<div class="flex flex-col gap-8 md:flex-row">
	<div class="w-full md:w-1/2">
		<h2 class="text-primary mb-6 text-2xl font-bold">Manage Sessions</h2>

		<form class="border-base-300 bg-base-200 mb-8 rounded border p-6 shadow-sm">
			<p class="text-secondary mb-2 font-semibold">
				{isEditing ? 'Edit Session' : 'Create New Session'}
			</p>
			<div class="flex flex-col gap-3 md:flex-row md:items-end">
				<input
					bind:value={form.name}
					type="text"
					placeholder="Session name"
					class="input input-bordered w-full"
				/>
				<input bind:value={form.start_date} type="date" class="input input-bordered w-full" />
				<input bind:value={form.end_date} type="date" class="input input-bordered w-full" />
				<button onclick={addOrUpdateSession} class="btn btn-primary w-full md:w-auto">
					{isEditing ? 'Update' : 'Add'}
				</button>
				{#if isEditing}
					<button onclick={resetForm} class="btn btn-ghost w-full md:w-auto">Cancel</button>
				{/if}
			</div>
		</form>

		<div class="border-base-300 bg-base-200 rounded border p-5 shadow-inner">
			<p class="text-secondary mb-2 font-semibold">All Sessions</p>
			{#if sessions.data.length > 0}
				<ul class="max-h-[65vh] space-y-4 overflow-y-auto">
					{#each sessions.data as session (session.id)}
						<li
							class={` border-accent flex items-center justify-between rounded-none p-4  shadow-sm hover:cursor-pointer ${session.id === sessions.selected ? 'border-primary' : 'border-base-300'} border`}
							onclickcapture={() => {
								sessions.select(session.id);
							}}
						>
							<div>
								<p class="text-primary font-semibold">{session.name}</p>
								<p class="text-secondary text-sm">
									{session.start_date} → {session.end_date}
								</p>
							</div>
							<div class="flex gap-2">
								<button onclick={() => editSession(session)} class="btn btn-xs btn-info">
									Edit
								</button>
								<button onclick={() => removeSession(session.id)} class="btn btn-xs btn-error">
									Delete
								</button>
							</div>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="text-secondary text-sm">No sessions added yet.</p>
			{/if}
		</div>
	</div>

	<div class="w-1/2 pt-10">
		<div class="border-accent text-accent flex flex-col rounded-none border p-6 shadow-md">
			<h2 class="text-primary mb-4 text-xl font-bold">Session Overview</h2>
			<div class="space-y-3 text-sm">
				{#if sessions.selected}
					<div>
						<p class="text-secondary">Name</p>
						<p class="text-base font-semibold">{selectedSession?.name}</p>
					</div>
					<div>
						<p class="text-secondary">Start Date</p>
						<p class="text-base font-semibold">{selectedSession?.start_date}</p>
					</div>
					<div>
						<p class="text-secondary">End Date</p>
						<p class="text-base font-semibold">{selectedSession?.end_date}</p>
					</div>
					<hr class="border-base-300 my-4" />
					<div class="grid grid-cols-2 gap-4 text-sm">
						<div>
							<p class="text-secondary">Total Students</p>
							<p class="text-base font-semibold">[DUMMY DATA]</p>
						</div>
						<div>
							<p class="text-secondary">Total Classes</p>
							<p class="text-base font-semibold">[DUMMY DATA]</p>
						</div>
						<div>
							<p class="text-secondary">Incoming Money</p>
							<p class="text-success text-base font-semibold">{formatMoney(10000)}</p>
						</div>
						<div>
							<p class="text-secondary">Outgoing Costs</p>
							<p class="text-error text-base font-semibold">{formatMoney(10000)}</p>
						</div>
						<div class="col-span-2">
							<p class="text-secondary">Unpaid Money</p>
							<p class="text-info text-base font-semibold">{formatMoney(10000)}</p>
						</div>
					</div>
				{:else}
					<div>
						<p>No session is selected.</p>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<Toast />
