<script lang="ts">
	import Icon from '@iconify/svelte';
	import Guardians from './Guardians.svelte';
	import { onMount } from 'svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { students } from '$lib/store/student.svelte';
	import { invoke } from '@tauri-apps/api/core';

	import type { Student } from '$lib/types/student';
	import { sessions } from '$lib/store/session.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import Toast from '../global/Toast.svelte';
	import { guardians, studentRelationships } from '$lib/store/guardian.svelte';
	import type { StudentRelationship } from '$lib/types/guardian';

	let { isEditing = $bindable(false), selectedStudentData } = $props();

	type Guardian = {
		id: number;
		name: string;
		relation: string;
		relation_id?: number;
		phone: string;
		address: string;
		photo: string;
	};

	let selectedGuardians: Guardian[] = $state([]);
	let selectedGuardians_s: Guardian[] = $state([]);

	function removeGuardian(id: number) {
		selectedGuardians = selectedGuardians.filter((g) => g.id !== id);
	}

	const today = new Date();
	const localISOString = new Date(
		today.getTime() - today.getTimezoneOffset() * 60000
	).toISOString();

	let form_data = $state({
		name: '',
		class_id: '',
		section_id: '',
		dob: '',
		gender: '',
		religion: '',
		address: '',
		phone: '',
		admission_date: localISOString.split('T')[0],
		is_resident: false,
		roll: '',
		photo: '',
		health_notes: '',
		general_notes: ''
	});

	$effect(() => {
		if (isEditing && selectedStudentData) {
			form_data.name = selectedStudentData.name ?? '';
			form_data.class_id = selectedStudentData.class_id ?? '';
			form_data.section_id = selectedStudentData.section_id ?? '';
			form_data.dob = selectedStudentData.dob ?? '';
			form_data.gender = selectedStudentData.gender ?? '';
			form_data.religion = selectedStudentData.religion ?? '';
			form_data.address = selectedStudentData.address ?? '';
			form_data.phone = selectedStudentData.phone?.slice(4) ?? '';
			form_data.admission_date = selectedStudentData.admission_date ?? '';
			form_data.is_resident = selectedStudentData.is_resident ?? false;
			form_data.roll = selectedStudentData.roll?.toString() ?? '';
			form_data.photo = selectedStudentData.photo ?? '';
			form_data.health_notes = selectedStudentData.health_notes ?? '';
			form_data.general_notes = selectedStudentData.general_notes ?? '';

			(async () => {
				if (selectedStudentData.id) {
					await studentRelationships.fetch(selectedStudentData.id);
					const srls = studentRelationships.get(selectedStudentData.id);

					const guardiansList: Guardian[] = srls.map((srl) => ({
						...(guardians.get(srl.related_id) as Guardian),
						relation_id: srl.id,
						relation: srl.relationship as string
					}));

					selectedGuardians = guardiansList;
					selectedGuardians_s = guardiansList;
				}
			})();
		} else {
			form_data = {
				name: '',
				class_id: '',
				section_id: '',
				dob: '',
				gender: '',
				religion: '',
				address: '',
				phone: '',
				admission_date: localISOString.split('T')[0],
				is_resident: false,
				roll: '',
				photo: '',
				health_notes: '',
				general_notes: ''
			};
			selectedGuardians = [];
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

	function openGuardiansModal() {
		(document.getElementById('create-student-modal') as HTMLDialogElement).close();
		(document.getElementById('manage-guardians-modal') as HTMLDialogElement).showModal();
	}

	async function submitStudentForm() {
		invoke(isEditing ? 'edit_student' : 'create_student', {
			id: isEditing ? selectedStudentData?.id : undefined,
			name: form_data.name,
			class_id: Number(form_data.class_id),
			section_id: form_data.section_id
				? Number(form_data.section_id) >= 0
					? Number(form_data.section_id)
					: null
				: null,
			session_id: sessions.selected,
			dob: form_data.dob,
			gender: form_data.gender,
			religion: form_data.religion,
			address: form_data.address,
			phone: form_data.phone ? '+880' + form_data.phone : null,
			admission_date: form_data.admission_date,
			is_resident: form_data.is_resident,
			roll: isEditing ? Number(form_data.roll) : -1,
			photo: form_data.photo || null,
			health_notes: form_data.health_notes || null,
			general_notes: form_data.general_notes || null
		})
			.then(async (student) => {
				if (isEditing) {
                    console.log("Sending for update, ", students.section_id)
					students.update(student as Student);

					const guardianForDelete = selectedGuardians_s.filter(
						(g1) => !selectedGuardians.some((g2) => g2.id === g1.id)
					);

					const guardianForAdd = selectedGuardians.filter(
						(g1) => !selectedGuardians_s.some((g2) => g2.id === g1.id)
					);

					for (const guardian of guardianForDelete) {
						if (guardian.relation_id) {
							await invoke('delete_student_relationship', {
								id: guardian.relation_id
							});
							studentRelationships.remove(guardian.id);
						}
					}

					for (const guardian of guardianForAdd) {
						const relation: StudentRelationship = await invoke('create_student_relationship', {
							student_id: (student as Student).id!,
							related_id: guardian.id,
							relationship: guardian.relation || null
						});
						studentRelationships.add(relation);
					}

					toast.set({ message: 'Student updated successfully!', type: 'success' });
				} else {
					students.add(student as Student);

					for (const guardian of selectedGuardians) {
						const relation: StudentRelationship = await invoke('create_student_relationship', {
							student_id: (student as Student).id!,
							related_id: guardian.id,
							relationship: guardian.relation || null
						});
						studentRelationships.add(relation);
					}

					toast.set({ message: 'Student created successfully!', type: 'success' });
				}

				console.log('edited: ', $state.snapshot(selectedGuardians));
				console.log('Constant: ', $state.snapshot(selectedGuardians_s));

				toast.set({ message: 'Student created successfully!', type: 'success' });

				form_data.name = '';
				form_data.class_id = '';
				form_data.section_id = '';
				form_data.dob = '';
				form_data.gender = '';
				form_data.religion = '';
				form_data.address = '';
				form_data.phone = '';
				form_data.admission_date = localISOString.split('T')[0];
				form_data.is_resident = false;
				form_data.roll = '';
				form_data.photo = '';
				form_data.health_notes = '';
				form_data.general_notes = '';

				isEditing = false;
			})
			.catch((err) => {
				console.error(err);

				if (typeof err === 'string' && err.includes('UNIQUE constraint failed')) {
					if (err.includes('students.roll')) {
						toast.set({ message: 'Roll number must be unique', type: 'error' });
					} else {
						toast.set({ message: 'Duplicate entry detected', type: 'error' });
					}
				} else {
					toast.set({
						message: isEditing ? 'Failed to update student' : 'Failed to create student',
						type: 'error'
					});
				}
			});
	}

	onMount(async () => {
		classes.fetch(sessions.selected as number);
	});
</script>

<dialog id="create-student-modal" class="modal">
	<div class="modal-box w-11/12 max-w-3xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<h3 class="mb-4 text-lg font-bold">{isEditing ? 'Edit' : 'Create'} Student</h3>

		<form on:submit|preventDefault={submitStudentForm} class="space-y-2">
			{#if classes.get_by_current_session().length === 0}
				<div class="alert alert-warning">Please create a class first</div>
			{:else}
				<div class="grid grid-cols-1 gap-3 md:grid-cols-2">
					<!-- Name -->
					<div>
						<label class="mb-1 block text-sm font-medium">Name</label>
						<input
							class="input input-bordered w-full"
							bind:value={form_data.name}
							required
							placeholder="Enter student name"
						/>
					</div>

					<!-- Class -->
					<div>
						<label class="mb-1 block text-sm font-medium">Class</label>
						<select class="input input-bordered w-full" bind:value={form_data.class_id} required>
							<option value="">Select Class</option>
							{#each classes.get_by_current_session() as cls, i (i)}
								<option value={cls.id}>{cls.name}</option>
							{/each}
						</select>
					</div>

					<!-- Section -->
					{#if sections.get_by_class(Number(form_data.class_id)).length > 0}
						<div>
							<label class="mb-1 block text-sm font-medium">Section</label>
							<select
								class="input input-bordered w-full"
								bind:value={form_data.section_id}
								required
							>
								<option value="">Select Section</option>
								{#each sections.get_by_class(Number(form_data.class_id)) as section, i (i)}
									<option value={section.id}>{section.name}</option>
								{/each}
							</select>
						</div>
					{/if}

					<!-- Date of Birth -->
					<div>
						<label class="mb-1 block text-sm font-medium">Date of Birth</label>
						<input
							type="date"
							class="input input-bordered w-full"
							bind:value={form_data.dob}
							required
						/>
					</div>

					<!-- Address -->
					<div>
						<label class="mb-1 block text-sm font-medium">Address</label>
						<input
							class="input input-bordered w-full"
							bind:value={form_data.address}
							required
							placeholder="Enter address"
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
						<label class="mb-1 block text-sm font-medium">Gender</label>
						<select class="select w-full" bind:value={form_data.gender}>
							<option disabled selected value="">Select gender</option>
							<option value="Male">Male</option>
							<option value="Female">Female</option>
							<option value="Other">Other</option>
						</select>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium">Religion</label>
						<select class="select w-full" bind:value={form_data.religion}>
							<option disabled selected value="">Select religion</option>
							<option value="Islam">Islam</option>
							<option value="Hinduism">Hinduism</option>
							<option value="Christianity">Christianity</option>
							<option value="Buddhism">Buddhism</option>
							<option value="Other">Other</option>
						</select>
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

					<!-- Health Notes -->
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium">Health Notes</label>
						<textarea
							class="textarea textarea-bordered w-full"
							bind:value={form_data.health_notes}
							placeholder="Enter any health-related notes"
						></textarea>
					</div>

					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium">General Notes</label>
						<textarea
							class="textarea textarea-bordered w-full"
							bind:value={form_data.general_notes}
							placeholder="Enter any general notes"
						></textarea>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							class="checkbox checkbox-sm"
							bind:checked={form_data.is_resident}
						/>
						<span>Is Resident</span>
					</div>

					<!-- Guardians -->
					{#if selectedGuardians.length > 0}
						<div class="bg-base-200 rounded p-4 md:col-span-2">
							<p class="mb-2 text-sm text-gray-500">Guardians</p>
							<ul class="grid max-h-48 grid-cols-2 gap-4 overflow-y-auto pr-1">
								{#each selectedGuardians as g, i ((g.id, i))}
									<li class="bg-base-300 flex items-center gap-4 rounded p-2 shadow-sm">
										<div class="size-12 flex-shrink-0 overflow-hidden rounded-full">
											<img
												src={g.photo}
												alt={`Photo of ${g.name}`}
												class="h-full w-full object-cover object-center"
											/>
										</div>
										<div class="min-w-0 flex-1">
											<p class="truncate font-medium">{g.name}</p>
											<p class="truncate text-sm text-gray-500">{g.relation ?? '—'} • {g.phone}</p>
											<p class="truncate text-sm text-gray-500">{g.address ?? 'No address'}</p>
										</div>
										<button
											type="button"
											class="btn btn-square btn-ghost"
											on:click={() => removeGuardian(g.id)}
											title="Remove Guardian"
											aria-label="Remove Guardian"
										>
											<Icon icon="fa:remove" />
										</button>
									</li>
								{/each}
							</ul>
						</div>
					{:else}
						<div class="alert alert-info md:col-span-2">
							<span>No guardians selected yet</span>
						</div>
					{/if}
				</div>

				<div class="modal-action mt-6 flex justify-between">
					<button class="btn btn-secondary btn-sm" on:click|preventDefault={openGuardiansModal}>
						Manage Guardians
					</button>
					<button type="submit" class="btn btn-primary btn-sm"
						>{isEditing ? 'Edit' : 'Create'}</button
					>
				</div>
			{/if}
		</form>
	</div>
	<Toast />
</dialog>

<Guardians bind:guardians={selectedGuardians} />
