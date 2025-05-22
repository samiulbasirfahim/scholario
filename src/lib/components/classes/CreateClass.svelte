<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';

	import type { Class } from '$lib/types/class';
	import { classes } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';

	let formData = $state({
		name: '',
		level: '',
		admission_fee: '',
		monthly_fee: '',
		readmission_fee: ''
	});
	const submitForm = () => {
		console.log({
			name: formData.name.trim(),
			level: Number(formData.level),
			admission_fee: Number(formData.admission_fee) * 100,
			monthly_fee: Number(formData.monthly_fee) * 100,
			readmission_fee: Number(formData.readmission_fee) * 100,
			session_id: sessions.selectedSession?.id
		});
		invoke('create_class', {
			name: formData.name.trim(),
			level: Number(formData.level),
			admission_fee: Number(formData.admission_fee) * 100,
			monthly_fee: Number(formData.monthly_fee) * 100,
			readmission_fee: Number(formData.readmission_fee) * 100,
			session_id: sessions.selectedSession?.id
		})
			.then((cls) => {
				classes.add(sessions.selectedSession?.id as number, cls as Class);
				toast.set({ message: 'Class created successfully', type: 'success' });

				formData.name = '';
				formData.level = '';
				formData.admission_fee = '';
				formData.monthly_fee = '';
				formData.readmission_fee = '';

				(document.getElementById('create-class-modal') as HTMLDialogElement).close();
			})
			.catch((err) => {
				if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
					if (err.includes('classes.name')) {
						toast.set({ message: 'Class name must be unique', type: 'error' });
					} else {
						toast.set({ message: 'Duplicate entry detected', type: 'error' });
					}
				} else {
					toast.set({ message: 'Failed to create class', type: 'error' });
				}
				console.error(err);
			});
	};
</script>

<dialog id="create-class-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="mb-4 text-lg font-bold">Create Class</h3>

		{#if sessions.selectedSession?.id}
			<form
				onsubmit={(e) => {
					e.preventDefault();
					submitForm();
				}}
				class="space-y-2"
			>
				<div class="form-control w-full">
					<label class="label" for="name">
						<span class="label-text font-semibold">Name</span>
					</label>
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
					<label class="label" for="level">
						<span class="label-text font-semibold">Level</span>
					</label>
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
					<label class="label" for="admission_fee">
						<span class="label-text font-semibold">Admission Fee</span>
					</label>
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
					<label class="label" for="monthly_fee">
						<span class="label-text font-semibold">Monthly Fee</span>
					</label>
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
					<label class="label" for="readmission_fee">
						<span class="label-text font-semibold">Re-admission Fee</span>
					</label>
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
					<button type="submit" class="btn btn-primary px-8"> Create </button>
				</div>
			</form>
		{:else}
			<div class="alert alert-warning">Please create a session first</div>
		{/if}
	</div>
	<form method="dialog" class="modal-backdrop">
		<button>close</button>
	</form>
	<Toast />
</dialog>
