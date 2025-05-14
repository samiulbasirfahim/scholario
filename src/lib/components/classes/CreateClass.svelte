<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '../global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte';

	import type { Class } from '$lib/types/class';
	import { classes } from '$lib/store/class.svelte';

	let formData = $state({
		name: '',
		level: '',
		admission_fee: '',
		monthly_fee: '',
		readmission_fee: ''
	});

	const submitForm = () => {
		invoke('create_class', {
			name: formData.name.trim(),
			level: Number(formData.level),
			admission_fee: Number(formData.admission_fee) * 100,
			monthly_fee: Number(formData.monthly_fee) * 100,
			readmission_fee: Number(formData.readmission_fee) * 100
		})
			.then((cls) => {
				classes.add(cls as Class);
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
	<div class="modal-box w-full max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">Create Class</h3>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				submitForm();
			}}
			class="space-y-4"
		>
			<div class="form-control">
				<label class="label" for="name">
					<span class="label-text">Name</span>
				</label>
				<input
					type="text"
					class="input input-bordered w-full"
					required
					bind:value={formData.name}
				/>
			</div>

			<div class="form-control">
				<label class="label" for="level">
					<span class="label-text">Level</span>
				</label>
				<input
					type="number"
					class="input input-bordered w-full"
					required
					bind:value={formData.level}
				/>
			</div>

			<div class="form-control">
				<label class="label" for="admission_fee">
					<span class="label-text">Admission Fee</span>
				</label>
				<input
					type="number"
					class="input input-bordered w-full"
					required
					bind:value={formData.admission_fee}
				/>
			</div>

			<div class="form-control">
				<label class="label" for="monthly_fee">
					<span class="label-text">Monthly Fee</span>
				</label>
				<input
					type="number"
					class="input input-bordered w-full"
					required
					bind:value={formData.monthly_fee}
				/>
			</div>

			<div class="form-control">
				<label class="label" for="readmission_fee">
					<span class="label-text">Re-admission Fee</span>
				</label>
				<input
					type="number"
					class="input input-bordered w-full"
					required
					bind:value={formData.readmission_fee}
				/>
			</div>

			<div class="flex justify-end">
				<button type="submit" class="btn btn-primary">Create</button>
			</div>
		</form>
	</div>
	<Toast />
</dialog>
