<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';

	import type { Section } from '$lib/types/class';

	import { classes, sections } from '$lib/store/class.svelte';

	let formData = $state({
		class_id: '',
		name: ''
	});

	const submitForm = () => {
		invoke('create_section', {
			class_id: Number(formData.class_id),
			name: formData.name.trim()
		})
			.then((section) => {
				sections.add(section as Section);
				toast.set({ message: 'Successfully created section', type: 'success' });
				formData.class_id = '';
				formData.name = '';
				(document.getElementById('create-section-modal') as HTMLDialogElement).close();
			})
			.catch((err: string) => {
				if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
					toast.set({
						message: 'Section name must be unique for the selected class',
						type: 'error'
					});
				} else {
					toast.set({ message: 'Failed to create section', type: 'error' });
				}
				console.error(err);
			});
	};
</script>

<dialog id="create-section-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="mb-4 text-lg font-bold">Create Section</h3>

		{#if classes.get_by_current_session().length > 0}
			<form
				onsubmit={(e) => {
					e.preventDefault();
					submitForm();
				}}
				class="space-y-2"
			>
				<div class="form-control w-full">
					<label class="label" for="class">
						<span class="label-text text-xs font-semibold">Class</span>
					</label>
					<select
						id="class"
						class="select select-bordered w-full"
						required
						bind:value={formData.class_id}
					>
						<option value="" disabled selected>Select class</option>
						{#each classes.get_by_current_session() as cls (cls.id)}
							<option value={cls.id}>{cls.name}</option>
						{/each}
					</select>
				</div>

				<div class="form-control w-full">
					<label class="label" for="name">
						<span class="label-text text-xs font-semibold">Section Name</span>
					</label>
					<input
						id="name"
						type="text"
						class="input input-bordered w-full"
						required
						bind:value={formData.name}
						placeholder="Enter section name"
					/>
				</div>

				<div class="flex justify-end pt-4">
					<button type="submit" class="btn btn-primary px-8"> Create </button>
				</div>
			</form>
		{:else}
			<p class="text-secondary text-sm">No classes yet. Click 'Create Class' to get started!</p>
		{/if}
	</div>

	<Toast />
</dialog>
