<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';

	import type { Class } from '$lib/types/class';
	import { classes } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';

	let { isEditing = $bindable(false), selectedClass } = $props();

	let selectedClassData = $derived(classes.get(sessions.selected as number, selectedClass));

	let formData = $state({
		name: '',
		level: '',
		admission_fee: '',
		monthly_fee: '',
		readmission_fee: ''
	});

	$effect(() => {
		if (isEditing && selectedClassData) {
			formData.name = isEditing ? selectedClassData?.name : '';
			formData.level = isEditing ? selectedClassData?.level.toString() : '';
			formData.admission_fee = isEditing ? (selectedClassData?.admission_fee / 100).toString() : '';
			formData.monthly_fee = isEditing ? (selectedClassData?.monthly_fee / 100).toString() : '';
			formData.readmission_fee = isEditing
				? (selectedClassData?.readmission_fee / 100).toString()
				: '';
		} else {
			formData.name = '' as string;
			formData.level = '' as string;
			formData.admission_fee = '' as string;
			formData.monthly_fee = '' as string;
			formData.readmission_fee = '' as string;
		}
	});

	const submitForm = () => {
		invoke(isEditing ? 'edit_class' : 'create_class', {
			id: selectedClassData?.id as number,
			name: (formData.name as string).trim(),
			level: Number(formData.level as string),
			admission_fee: Number(formData.admission_fee as string) * 100,
			monthly_fee: Number(formData.monthly_fee as string) * 100,
			readmission_fee: Number(formData.readmission_fee as string) * 100,
			session_id: sessions.selectedSession?.id as number
		})
			.then((cls) => {
				if (isEditing) {
					classes.update(sessions.selected as number, selectedClass, cls as Class);
					toast.set({ message: 'Class updated successfully', type: 'success' });
				} else {
					classes.add(sessions.selectedSession?.id as number, cls as Class);
					toast.set({ message: 'Class created successfully', type: 'success' });
				}

				formData.name = '';
				formData.level = '';
				formData.admission_fee = '';
				formData.monthly_fee = '';
				formData.readmission_fee = '';

				(document.getElementById('create-class-modal') as HTMLDialogElement).close();
				isEditing = false;
			})
			.catch((err) => {
				if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
					if (err.includes('classes.name')) {
						toast.set({ message: 'Class name must be unique', type: 'error' });
					} else {
						toast.set({ message: 'Duplicate entry detected', type: 'error' });
					}
				} else {
					if (isEditing) toast.set({ message: 'Failed to update class', type: 'error' });
					else toast.set({ message: 'Failed to create class', type: 'error' });
				}
				console.error(err);
			});
	};
</script>

<dialog id="create-class-modal" class="modal">
	<div class="modal-box backdrop-blur-sm">
		<form method="dialog">
			<button
				class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2"
				on:click|preventDefault={() => {
					(document.getElementById('create-class-modal') as HTMLDialogElement).close();
					isEditing = false;
				}}>✕</button
			>
		</form>
		<h3 class="mb-4 text-lg font-bold">{isEditing ? 'Edit ' : 'Create '} Class</h3>

		{#if sessions.selectedSession?.id}
			<form on:submit|preventDefault={submitForm} class="space-y-2">
				<div class="form-control w-full">
					<label class="mb-1 block text-sm font-medium">Name</label>
					<input
						id="name"
						type="text"
						class="input input-bordered w-full"
						required
						bind:value={formData.name}
						placeholder="Enter class name"
					/>
				</div>

				<div class="form-control w-full">
					<label class="mb-1 block text-sm font-medium">Level</label>
					<input
						id="level"
						type="number"
						class="input input-bordered w-full"
						required
						bind:value={formData.level}
						min="1"
						placeholder="Enter level (number)"
					/>
				</div>

				<div class="form-control w-full">
					<label class="mb-1 block text-sm font-medium">Admission Fee</label>
					<input
						id="admission_fee"
						type="number"
						class="input input-bordered w-full"
						required
						bind:value={formData.admission_fee}
						min="0"
						step="0.01"
						placeholder="Enter admission fee"
					/>
				</div>

				<div class="form-control w-full">
					<label class="mb-1 block text-sm font-medium">Monthly Fee</label>
					<input
						id="monthly_fee"
						type="number"
						class="input input-bordered w-full"
						required
						bind:value={formData.monthly_fee}
						min="0"
						step="0.01"
						placeholder="Enter monthly fee"
					/>
				</div>

				<div class="form-control w-full">
					<label class="mb-1 block text-sm font-medium">Re-admission Fee</label>
					<input
						id="readmission_fee"
						type="number"
						class="input input-bordered w-full"
						required
						bind:value={formData.readmission_fee}
						min="0"
						step="0.01"
						placeholder="Enter re-admission fee"
					/>
				</div>

				<div class="flex justify-end pt-4">
					<button type="submit" class="btn btn-primary px-8"
						>{isEditing ? 'Save' : 'Create'}
					</button>
				</div>
			</form>
		{:else}
			<div class="alert alert-warning">Please create a session first</div>
		{/if}
	</div>

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button aria-label="Close modal" class="h-full w-full" />
	</form>
	<Toast />
</dialog>
