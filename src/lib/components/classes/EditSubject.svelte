<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { classSubjects, subjects } from '$lib/store/class.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';
	import type { Subject } from '$lib/types/class';

	const { selectedSubject } = $props();

	let formData = $state({
		id: 0,
		name: '',
		code: ''
	});

	$effect(() => {
		let s = subjects.get(selectedSubject);
		if (s) {
			formData = { ...s, code: s.code as unknown as string };
		}
	});

	const updateSubject = async () => {
		console.log(formData);
		if (!formData.name?.trim() || Number(formData.code) < 1) {
			toast.set({ message: 'Subject name and code are required', type: 'warning' });
			return;
		}

		try {
			const updated = await invoke('edit_subject', {
				...formData,
				code: Number(formData.code)
			});

			subjects.update(updated as Subject);

			toast.set({ message: 'Subject updated successfully', type: 'success' });
			(document.getElementById('edit-subject-modal') as HTMLDialogElement).close();
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to update subject', type: 'error' });
		}
	};

	const deleteSubject = async () => {
		try {
			console.log(selectedSubject);
			await invoke('delete_subject', { id: selectedSubject });
			subjects.remove(selectedSubject);
			classSubjects.remove_from_all(selectedSubject);
			toast.set({ message: 'Subject deleted', type: 'success' });
			(document.getElementById('edit-subject-modal') as HTMLDialogElement).close();
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to delete subject', type: 'error' });
		}
	};
</script>

<dialog id="edit-subject-modal" class="modal">
	<div class="modal-box w-96">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<h3 class="mb-4 text-lg font-bold">Edit Subject</h3>

		<form class="space-y-4">
			<div>
				<label for="subjectName" class="block text-sm font-medium">Subject Name</label>
				<input
					id="subjectName"
					type="text"
					class="input input-bordered w-full"
					bind:value={formData.name}
				/>
			</div>

			<div>
				<label for="subjectCode" class="block text-sm font-medium">Subject Code</label>
				<input
					id="subjectCode"
					type="number"
					class="input input-bordered w-full"
					bind:value={formData.code}
				/>
			</div>

			<div class="join mt-4 flex justify-end">
				<button type="submit" class="btn btn-primary join-item" onclick={updateSubject}>
					Update
				</button>
				<button class="btn btn-error join-item" onclick={deleteSubject}>Delete</button>
			</div>
		</form>
	</div>

	<Toast />
</dialog>
