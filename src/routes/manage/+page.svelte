<script lang="ts">
	import Toast from '$lib/components/global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';

	let sessions: { name: string; start: string; end: string }[] = $state([]);

	function formatDate(date: Date): string {
		return date.toISOString().split('T')[0];
	}

	const today = new Date();
	const oneYearLater = new Date(today);
	oneYearLater.setFullYear(today.getFullYear() + 1);

	let form = $state({
		name: '',
		start: formatDate(today),
		end: formatDate(oneYearLater)
	});

	function addSession() {
		if (form.name.trim() && form.start && form.end) {
			sessions = [
				...sessions,
				{
					name: form.name.trim(),
					start: form.start,
					end: form.end
				}
			];

			form = {
				name: '',
				start: formatDate(new Date()),
				end: formatDate(new Date(new Date().setFullYear(new Date().getFullYear() + 1)))
			};

			toast.set({ message: 'Session created successfully', type: 'success' });
		} else {
			toast.set({ message: 'Please fill in all required fields', type: 'error' });
		}
	}

	function removeSession(index: number) {
		sessions.splice(index, 1);
		sessions = [...sessions];
	}
</script>

<div class="flex flex-col gap-8 md:flex-row">
	<div class="w-full md:w-1/2">
		<h2 class="text-primary mb-6 text-2xl font-bold">Manage Sessions</h2>

		<form class="border-base-300 bg-base-200 mb-8 rounded border p-6 shadow-sm">
			<p class="text-secondary mb-2 font-semibold">Create New Session</p>
			<div class="flex flex-col gap-3 md:flex-row md:items-end">
				<input
					bind:value={form.name}
					type="text"
					placeholder="Session name"
					class="input input-bordered w-full"
				/>
				<input bind:value={form.start} type="date" class="input input-bordered w-full" />
				<input bind:value={form.end} type="date" class="input input-bordered w-full" />
				<button onclick={addSession} class="btn btn-primary w-full md:w-auto"> Add </button>
			</div>
		</form>

		<div class="border-base-300 bg-base-200 rounded border p-5 shadow-inner">
			<p class="text-secondary mb-2 font-semibold">All Sessions</p>
			{#if sessions.length > 0}
				<ul class="space-y-3">
					{#each sessions as session, index (index)}
						<li
							class="border-accent bg-base-100 flex items-center justify-between rounded border p-4 shadow-sm"
						>
							<div>
								<p class="text-primary font-semibold">{session.name}</p>
								<p class="text-secondary text-sm">
									{session.start} → {session.end}
								</p>
							</div>
							<button onclick={() => removeSession(index)} class="btn btn-xs btn-error">
								Delete
							</button>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="text-secondary text-sm">No sessions added yet.</p>
			{/if}
		</div>
	</div>

	<!-- Right Column: Placeholder -->
	<div
		class="border-accent text-accent hidden w-1/2 items-center justify-center rounded border border-dashed p-4 md:flex"
	>
		<p>Right half reserved for future content</p>
	</div>
</div>

<Toast />
