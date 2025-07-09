<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { guardians } from '$lib/store/guardian.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Guardian } from '$lib/types/guardian';
	import Toast from '../global/Toast.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let { guardians: guardians_selected = $bindable() } = $props();

	let guardian_selected: Guardian | null = $state(null);
	let form_data = $state({
		name: '',
		address: '',
		phone: '',
		photo: ''
	});

	let relation = $state('');

	let rels = $state(['father', 'mother', 'sister', 'brother']);

	let searchTerm = $state('');

	function closeGuardiansModal() {
		const studentModal = document.getElementById('create-student-modal') as HTMLDialogElement;
		const guardianModal = document.getElementById('manage-guardians-modal') as HTMLDialogElement;

		guardianModal.close();
		studentModal.showModal();
	}

	onMount(() => {
		guardians.fetch();
	});

	let searched_guardians = $state<Guardian[]>([]);

	$effect(() => {
		const q = searchTerm.trim();

		if (!q) {
			searched_guardians = guardians.data.slice(0, 9);
			return;
		}

		(async () => {
			try {
				searched_guardians = await invoke<Guardian[]>('search_guardians', { query: q });
			} catch (err) {
				console.error(err);
			}
		})();
	});

	async function submitForm() {
		try {
			let guardian = await invoke('create_guardian', {
				...form_data,
				phone: `+880${form_data.phone}`
			});
			guardians.add(guardian as Guardian);
			form_data = { name: '', phone: '', address: '', photo: '' };
			(document.getElementById('photo') as HTMLInputElement).value = '';
		} catch (err) {
			if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
				if (err.includes('phone')) {
					toast.set({ message: 'Phone number must be unique', type: 'error' });
				}
			} else {
				toast.set({ message: 'Failed to create guardian', type: 'error' });
			}
			console.log('Failed to create guardian', err);
		}
	}

	function handleFileUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files ? input.files[0] : null;
		if (file && file.type.startsWith('image/')) {
			const reader = new FileReader();
			reader.onload = function (e) {
				const base64String = e.target?.result as string;
				form_data.photo = base64String;
			};
			reader.readAsDataURL(file);
		} else {
			alert('Please select a valid image file.');
		}
	}
</script>

<dialog id="manage-guardians-modal" class="modal">
	<div class="modal-box w-11/12 max-w-3xl">
		<form method="dialog">
			<button
				class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2"
				onclick={(e) => {
					e.preventDefault();
					closeGuardiansModal();
				}}>✕</button
			>
		</form>
		<h3 class="mb-4 text-lg font-bold">Manage Guardians</h3>

		<div class="grid grid-cols-2 gap-6">
			<div class="space-y-2">
				<input
					type="text"
					id="name"
					name="name"
					class="input input-bordered w-full"
					placeholder="Search by name & phone"
					required
					bind:value={searchTerm}
				/>
				<ul class="list grid max-h-86 grid-cols-1 gap-4 overflow-y-auto">
					{#each searched_guardians.filter((g) => !guardians_selected.some((existing: Guardian) => existing.id === g.id)) as guardian, i (i)}
						<li
							class="list-row bg-base-300 rounded-box flex items-center justify-between gap-4 p-2"
						>
							<div class="shrink-0">
								<img
									class="size-12 rounded-full object-cover"
									src={guardian.photo}
									alt="{guardian.name}'s photo"
								/>
							</div>

							<div class="min-w-0 flex-1">
								<div class="truncate font-medium">{guardian.name}</div>
								<div class="truncate text-sm text-gray-500">
									{guardian.phone}
								</div>
								<div class="truncate text-sm text-gray-400">{guardian.address}</div>
							</div>

							<button
								class="btn btn-square btn-ghost btn-sm"
								onclick={(e) => {
									e.preventDefault();
									(
										document.getElementById('guardian_relationship') as HTMLDialogElement
									).showModal();
									(document.getElementById('manage-guardians-modal') as HTMLDialogElement).close();
									guardian_selected = guardian;
								}}
							>
								<Icon icon="fluent:add-12-filled" font-size="26" />
							</button>
						</li>
					{/each}
				</ul>

				{#if searched_guardians.filter((g) => !guardians_selected.some((existing: Guardian) => existing.id === g.id)).length < 1}
					<div class="alert alert-info col-span-2">
						<span>You haven't created any guardians yet, create one first</span>
					</div>
				{/if}
			</div>

			<div>
				<form
					class="space-y-4 px-4"
					onsubmit={(e) => {
						e.preventDefault();
						submitForm();
					}}
				>
					<p class="mb-4 text-lg font-bold">Create guardian</p>

					<div>
						<label for="name" class="block text-sm font-medium">Name</label>
						<input
							type="text"
							id="name"
							name="name"
							class="input input-bordered w-full"
							placeholder="Enter Guardian Name"
							required
							bind:value={form_data.name}
						/>
					</div>

					<div>
						<label for="name" class="block text-sm font-medium">Address</label>
						<input
							type="text"
							id="name"
							name="name"
							class="input input-bordered w-full"
							placeholder="Enter Guardian Address"
							required
							bind:value={form_data.address}
						/>
					</div>

					<div>
						<label for="phone" class="block text-sm font-medium">Phone</label>
						<label class="input validator w-full">
							+880
							<input
								type="tel"
								class="w-full tabular-nums"
								required
								pattern="[0-9]*"
								minlength="10"
								maxlength="10"
								title="Must be 10 digits"
								placeholder="10 digits"
								bind:value={form_data.phone}
							/>
						</label>
					</div>

					<div>
						<label for="photo" class="block text-sm font-medium">Photo</label>
						<input
							type="file"
							class="file-input input-bordered w-full"
							accept="image/*"
							id="photo"
							onchange={handleFileUpload}
						/>
					</div>

					<button type="submit" class="btn btn-primary w-full">Create</button>
				</form>
			</div>
		</div>
	</div>
	<Toast />
</dialog>

<dialog id="guardian_relationship" class="modal">
	<div class="modal-box">
		<h3 class="text-lg font-bold">Relationship</h3>
		<div class="modal-action flex flex-col items-end">
			<form class="w-full">
				<input
					type="text"
					class="input w-full"
					placeholder="Relation between student and guardian"
					list="relation"
					bind:value={relation}
				/>
				<datalist id="relation">
					{#each rels as rel, i (i)}
						<option value={rel}></option>
					{/each}
				</datalist>
			</form>
			<form method="dialog" class="join">
				<button
					class="btn btn-primary join-item btn-sm"
					onclick={(e) => {
						e.preventDefault();
						if (guardian_selected !== null) {
							guardians_selected.push({
								...(guardian_selected as Guardian),
								relation
							});
						}
						relation = '';
						(document.getElementById('manage-guardians-modal') as HTMLDialogElement).showModal();
						(document.getElementById('guardian_relationship') as HTMLDialogElement).close();
					}}>Save</button
				>
				<button
					class="btn btn-error join-item btn-sm"
					onclick={(e) => {
						e.preventDefault();
						(document.getElementById('manage-guardians-modal') as HTMLDialogElement).showModal();
						(document.getElementById('guardian_relationship') as HTMLDialogElement).close();
					}}>Cancel</button
				>
			</form>
		</div>
	</div>

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button>close</button>
	</form>

</dialog>
