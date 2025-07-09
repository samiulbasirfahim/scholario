<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { toast } from '$lib/store/toast.svelte';
	import { staff } from '$lib/store/staff.svelte';
	import type { Staff } from '$lib/types/staff';
	import { page } from '$app/state';

	let { isEditing = $bindable(false), selectedStaffData = null } = $props();

	const fakeRoles = [
		'Administrator',
		'Accountant',
		'Counselor',
		'Librarian',
		'Janitor',
		'Coach',
		'IT Support',
		'Nurse'
	];

	let form_data = $state({
		name: '',
		role: '',
		salary: '',
		qualification: '',
		phone: '',
		address: '',
		hire_date: '',
		photo: '',
		general_note: '',
		health_note: ''
	});

	$effect(() => {
		if (isEditing && selectedStaffData) {
			form_data.name = selectedStaffData.name ?? '';
			form_data.role = selectedStaffData.role ?? '';
			form_data.salary = selectedStaffData.salary?.toString() ?? '';
			form_data.qualification = selectedStaffData.qualification ?? '';
			form_data.phone = selectedStaffData.phone?.slice(4) ?? '';
			form_data.address = selectedStaffData.address ?? '';
			form_data.hire_date = selectedStaffData.hire_date ?? '';
			form_data.photo = selectedStaffData.photo ?? '';
			form_data.general_note = selectedStaffData.general_note ?? '';
			form_data.health_note = selectedStaffData.health_note ?? '';
		} else {
			form_data = {
				name: '',
				role: '',
				salary: '',
				qualification: '',
				phone: '',
				address: '',
				hire_date: '',
				photo: '',
				general_note: '',
				health_note: ''
			};
		}
	});

	function handleFileUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input?.files?.[0];
		if (file && file.type.startsWith('image/')) {
			const reader = new FileReader();
			reader.onload = (e) => {
				form_data.photo = e.target?.result as string;
			};
			reader.readAsDataURL(file);
		}
	}

	async function submitStaffForm() {
		try {
			let data = {
				id: isEditing ? selectedStaffData?.id : undefined,
				name: form_data.name,
				role: page.url.pathname === '/staffs' ? form_data.role : 'teacher',
				salary: Number(form_data.salary),
				qualification: form_data.qualification,
				is_teacher: page.url.pathname !== '/staffs',
				phone: form_data.phone ? '+880' + form_data.phone : null,
				address: form_data.address,
				hire_date: form_data.hire_date,
				photo: form_data.photo || null,
				general_note: form_data.general_note || null,
				health_note: form_data.health_note || null
			};
			const staffMember = await invoke<Staff>(isEditing ? 'update_staff' : 'create_staff', data);

			if (isEditing) {
				staff.update(data as Staff);
				toast.set({ message: 'Staff updated successfully!', type: 'success' });
			} else {
				staff.add(staffMember as Staff);
				toast.set({ message: 'Staff created successfully!', type: 'success' });
			}

			form_data = {
				name: '',
				role: '',
				salary: '',
				qualification: '',
				phone: '',
				address: '',
				hire_date: '',
				photo: '',
				general_note: '',
				health_note: ''
			};
			isEditing = false;
		} catch (err) {
			console.error(err);
			toast.set({
				message: isEditing ? 'Failed to update staff' : 'Failed to create staff',
				type: 'error'
			});
		}
	}
</script>

<dialog id="create-staff-modal" class="modal">
	<div class="modal-box w-11/12 max-w-3xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<h3 class="mb-4 text-lg font-bold">
			{isEditing ? 'Edit' : 'Create'}
			{page.url.pathname === '/teachers' ? 'Teacher' : 'Staff'}
		</h3>

		<form on:submit|preventDefault={submitStaffForm} class="grid grid-cols-2 gap-4">
			<div>
				<label class="mb-1 block text-sm font-medium">Name</label>
				<input
					class="input input-bordered w-full"
					bind:value={form_data.name}
					required
					placeholder="Enter staff name"
				/>
			</div>

			{#if page.url.pathname === '/staffs'}
				<div>
					<label class="mb-1 block text-sm font-medium">Role</label>
					<input
						type="text"
						class="input input-bordered w-full"
						list="roles"
						bind:value={form_data.role}
						placeholder="Enter or select role"
						required
					/>
					<datalist id="roles">
						{#each fakeRoles as role, i (i)}
							<option value={role}></option>
						{/each}
					</datalist>
				</div>
			{/if}

			<div>
				<label class="mb-1 block text-sm font-medium">Salary</label>
				<input
					type="number"
					class="input input-bordered w-full"
					bind:value={form_data.salary}
					min="0"
					required
					placeholder="Enter salary"
				/>
			</div>

			<div>
				<label class="mb-1 block text-sm font-medium">Qualification</label>
				<input
					class="input input-bordered w-full"
					bind:value={form_data.qualification}
					placeholder="Enter qualification"
				/>
			</div>

			<div>
				<label class="mb-1 block text-sm font-medium">Phone</label>
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
				<label class="mb-1 block text-sm font-medium">Address</label>
				<input
					class="input input-bordered w-full"
					bind:value={form_data.address}
					placeholder="Enter address"
				/>
			</div>

			<div>
				<label class="mb-1 block text-sm font-medium">Hire Date</label>
				<input
					type="date"
					class="input input-bordered w-full"
					bind:value={form_data.hire_date}
					required
				/>
			</div>

			<div>
				<label class="mb-1 block text-sm font-medium">Photo</label>
				<input
					type="file"
					class="file-input w-full"
					accept="image/*"
					on:change={handleFileUpload}
				/>
			</div>

			<div class="col-span-2">
				<label class="mb-1 block text-sm font-medium">General Note</label>
				<textarea
					class="textarea textarea-bordered w-full"
					bind:value={form_data.general_note}
					rows="3"
					placeholder="General notes about staff"
				></textarea>
			</div>

			<div class="col-span-2">
				<label class="mb-1 block text-sm font-medium">Health Note</label>
				<textarea
					class="textarea textarea-bordered w-full"
					bind:value={form_data.health_note}
					rows="3"
					placeholder="Health-related information"
				></textarea>
			</div>

			<div class="modal-action col-span-2 mt-6 flex justify-end">
				<button type="submit" class="btn btn-primary btn-sm">{isEditing ? 'Edit' : 'Create'}</button
				>
			</div>
		</form>
	</div>

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button>close</button>
	</form>
</dialog>
