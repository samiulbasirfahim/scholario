<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import { subjects } from '$lib/store/class.svelte';
	import type { Subject } from '$lib/types/class';

	let formData = $state({
		name: '',
		code: ''
	});

	const submitForm = () => {
		console.log(formData);
		invoke('create_subject', {
			name: formData.name.trim(),
			code: Number(formData.code)
		})
			.then((d) => {
				subjects.add(d as Subject);
				toast.set({ message: 'Successfully created subject', type: 'success' });
				formData.code = '';
				formData.name = '';
				(document.getElementById('create-subject-modal') as HTMLDialogElement).close();
			})
			.catch((err: string) => {
				if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
					if (err.includes('subjects.name')) {
						toast.set({ message: 'Subject name must be unique', type: 'error' });
					} else if (err.includes('subjects.code')) {
						toast.set({ message: 'Subject code must be unique', type: 'error' });
					} else {
						toast.set({ message: 'Duplicate entry detected', type: 'error' });
					}
				} else {
					toast.set({ message: 'Failed to create subject', type: 'error' });
				}
				console.error(err);
			});
	};
</script>

<dialog id="create-subject-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="mb-4 text-lg font-bold">Create Subject</h3>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				submitForm();
			}}
			class="space-y-4"
		>
			<div class="form-control">
				<label class="mb-1 block text-sm font-medium">Name</label>
				<input
					type="text"
					placeholder="Enter subject name"
					class="input input-bordered w-full"
					required
					bind:value={formData.name}
				/>
			</div>

			<div class="form-control">
				<label class="mb-1 block text-sm font-medium">Code</label>
				<input
					type="number"
					placeholder="Enter numeric code"
					class="input input-bordered w-full"
					required
					bind:value={formData.code}
				/>
			</div>

			<div class="flex justify-end">
				<button type="submit" class="btn btn-primary">Create</button>
			</div>
		</form>
	</div>
	<Toast />

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button aria-label="Close modal" class="h-full w-full" />
	</form>
</dialog>
