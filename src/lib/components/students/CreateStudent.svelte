<script lang="ts">
	import Icon from '@iconify/svelte';
	import Guardians from './Guardians.svelte';
	import { onMount } from 'svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { students, studentRelationships } from '$lib/store/student.svelte';
	import { invoke } from '@tauri-apps/api/core';

	import type { Student, StudentRelationship } from '$lib/types/student';
	import { sessions } from '$lib/store/session.svelte';

	type Guardian = {
		id: number;
		name: string;
		relation: string;
		phone: string;
		address: string;
		photo: string;
	};

	let guardians: Guardian[] = $state([]);

	function removeGuardian(id: number) {
		guardians = guardians.filter((g) => g.id !== id);
	}

	let form_data = $state({
		name: '',
		class_id: '',
		section_id: '',
		dob: '',
		gender: 'MALE',
		religion: 'ISLAM',
		address: '',
		phone: '',
		admission_date: new Date().toISOString().split('T')[0],
		is_resident: false,
		photo: '',
		health_notes: '',
		general_notes: ''
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

	async function submitStudentForm(e: Event) {
		e.preventDefault();

		try {
			const student: Student = await invoke('create_student', {
				name: form_data.name,
				classId: Number(form_data.class_id),
				sectionId: Number(form_data.section_id),
				sessionId: sessions.selected,
				dob: form_data.dob,
				gender: form_data.gender,
				religion: form_data.religion,
				address: form_data.address,
				phone: form_data.phone ? '+880' + form_data.phone : null,
				admissionDate: form_data.admission_date,
				isResident: form_data.is_resident,
				photo: form_data.photo || null,
				healthNotes: form_data.health_notes || null,
				generalNotes: form_data.general_notes || null
			});

			students.add(student);

			for (const guardian of guardians) {
				const relation: StudentRelationship = await invoke('create_student_relationship', {
					studentId: student.id!,
					relatedId: guardian.id,
					relationship: guardian.relation || null
				});
				studentRelationships.add(relation);
			}

			alert('Student created successfully!');
		} catch (err) {
			console.error(err);
			alert('Failed to create student');
		}
	}

	onMount(async () => {
		await sessions.fetch();
		classes.fetch(sessions.selected as number);
		sections.fetch();
	});
</script>

<dialog id="create-student-modal" class="modal">
	<div class="modal-box w-11/12 max-w-3xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<h3 class="mb-4 text-lg font-bold">Create Student</h3>

		<form on:submit|preventDefault={submitStudentForm} class="space-y-2">
			{#if classes.get_by_current_session().length === 0}
				<div class="alert alert-warning">Please create a class first</div>
			{:else}
				<div class="grid grid-cols-1 gap-2 md:grid-cols-2">
					<!-- Name -->
					<div>
						<label class="label">Name</label>
						<input
							class="input input-bordered w-full"
							bind:value={form_data.name}
							required
							placeholder="Enter student name"
						/>
					</div>

					<!-- Class -->
					<div>
						<label class="label">Class</label>
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
							<label class="label">Section</label>
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

					<div>
						<label class="label">Date of Birth</label>
						<input
							type="date"
							class="input input-bordered w-full"
							bind:value={form_data.dob}
							required
							placeholder="Enter date of birth"
						/>
					</div>

					<!-- Address -->
					<div>
						<label class="label">Address</label>
						<input
							class="input input-bordered w-full"
							bind:value={form_data.address}
							required
							placeholder="Enter address"
						/>
					</div>

					<!-- Phone -->
					<div class="w-full">
						<label class="label">Phone</label>
						<label class="input input-bordered flex w-full items-center gap-2">
							+880
							<input
								class="w-full grow"
								type="tel"
								placeholder="10 digits"
								pattern="[0-9]{10}"
								minlength="10"
								maxlength="10"
								bind:value={form_data.phone}
							/>
						</label>
					</div>

					<!-- Photo Upload -->
					<div>
						<label class="label">Photo</label>
						<input
							type="file"
							class="file-input w-full"
							accept="image/*"
							on:change={handleFileUpload}
							placeholder="Upload student photo"
						/>
					</div>

					<!-- Health Notes -->
					<div class="md:col-span-2">
						<label class="label">Health Notes</label>
						<textarea
							class="textarea textarea-bordered w-full"
							bind:value={form_data.health_notes}
							placeholder="Enter any health-related notes"
						></textarea>
					</div>

					<!-- General Notes -->
					<div class="md:col-span-2">
						<label class="label">General Notes</label>
						<textarea
							class="textarea textarea-bordered w-full"
							bind:value={form_data.general_notes}
							placeholder="Enter any general notes"
						></textarea>
					</div>

					<!-- Is Resident -->
					<div class="flex items-center gap-2">
						<input type="checkbox" class="checkbox" bind:checked={form_data.is_resident} />
						<span>Is Resident</span>
					</div>

					<!-- Guardians -->
					{#if guardians.length > 0}
						<div class="bg-base-200 rounded p-4 md:col-span-2">
							<p class="mb-2 text-sm text-gray-500">Guardians</p>
							<ul class="grid max-h-48 grid-cols-2 gap-4 overflow-y-auto pr-1">
								{#each guardians as g (g.id)}
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
					<button class="btn btn-secondary" on:click|preventDefault={openGuardiansModal}>
						Manage Guardians
					</button>
					<button type="submit" class="btn btn-primary">Create Student</button>
				</div>
			{/if}
		</form>
	</div>
</dialog>

<Guardians bind:guardians />
