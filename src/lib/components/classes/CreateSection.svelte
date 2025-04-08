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
	<div class="modal-box w-full max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">Create Section</h3>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				submitForm();
			}}
			class="space-y-4"
		>
			<div class="form-control">
				<label class="label" for="class">
					<span class="label-text">Class</span>
				</label>
				<select class="select select-bordered w-full" required bind:value={formData.class_id}>
					<option value="" disabled selected>Select class</option>
					{#each classes.classes as cls, i (i)}
						<option value={cls.id}>{cls.name}</option>
					{/each}
				</select>
			</div>

			<div class="form-control">
				<label class="label" for="name">
					<span class="label-text">Section Name</span>
				</label>
				<input
					type="text"
					class="input input-bordered w-full"
					required
					bind:value={formData.name}
				/>
			</div>

			<div class="flex justify-end">
				<button type="submit" class="btn btn-primary">Create</button>
			</div>
		</form>
	</div>
	<Toast />
</dialog>
