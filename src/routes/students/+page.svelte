<script lang="ts">
	import Navbar from '$lib/components/global/Navbar.svelte';
	import CreateStudent from '$lib/components/students/CreateStudent.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { guardians, studentRelationships } from '$lib/store/guardian.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Student } from '$lib/types/student';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import Toast from '$lib/components/global/Toast.svelte';
	import { toast } from '$lib/store/toast.svelte.js';
	const { data } = $props();

	type Guardian = {
		id: number;
		name: string;
		relation: string;
		phone: string;
		address: string;
		photo: string;
	};

	let selectedStudent = $state<number | null>(null);
	let selectedStudentData = $state<Student | null>();

	$effect(() => {
		if (selectedStudent) {
			goto('?selectedStudent=' + selectedStudent, { replaceState: true });
			selectedStudentData = students.getById(selectedStudent);
		}
	});

	onMount(() => {
		selectedStudent = Number(data.selectedStudent);
	});

	let guardians_s = $state<Guardian[]>([]);

	$effect(() => {
		console.log(studentRelationships.reactiveCounter);
		(async () => {
			if (selectedStudent) {
				await studentRelationships.fetch(selectedStudent);
				const srls = studentRelationships.get(selectedStudent);

				const guardiansList: Guardian[] = srls.map((srl) => ({
					...(guardians.get(srl.related_id) as Guardian),
					relation: srl.relationship as string
				}));

				guardians_s = guardiansList;
			}
		})();
	});

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: ''
	});

	let isEditing = $state(false);

	let students_d = $state<Student[]>([]);

	const deleteStudent = async () => {
		try {
			if (selectedStudent) {
				await invoke('delete_student', {
					id: selectedStudent,
					session_id: sessions.selected as number
				});
				students.remove(selectedStudent);
                selectedStudent = null;
				toast.set({ message: 'Student deleted', type: 'success' });
			}
		} catch (err) {
			console.log(err);
			toast.set({ message: 'Failed to delete student', type: 'error' });
		}
	};

	$effect(() => {
		const sessionId = sessions.selected as number;

		const classId = filter.class === '' ? undefined : Number(filter.class);
		const sectionId = filter.section === '' ? undefined : Number(filter.section);

		students.get(sessionId, classId, sectionId).then((d) => {
			students_d = d;
		});
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs font-semibold">
			<ul>
				<li>Students</li>

				{#if sessions.selectedSession}
					<li>{sessions.selectedSession.name.trim().split(' ')[0]}</li>
				{/if}

				{#if filter.class}
					<li>{classes.get(sessions.selected as number, Number(filter.class))?.name}</li>
				{/if}

				{#if filter.section}
					<li>{sections.get(Number(filter.section))?.name}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-2">
		<input
			type="text"
			placeholder="Search"
			class="input input-bordered w-48 transform-gpu transition-all ease-linear focus:w-64 focus:outline-none"
		/>

		<label class="bg-accent text-accent-content flex items-center rounded px-2">
			<Icon icon="carbon:prompt-session" font-size="20" />
			<select
				class="select border-0 bg-transparent focus:outline-none"
				bind:value={sessions.selected}
				on:change={(e) => sessions.select(Number((e.target as HTMLOptionElement).value))}
			>
				{#each sessions.data as session (session.id)}
					<option value={session.id}>{session.name}</option>
				{/each}
			</select>
		</label>

		<button
			class="btn btn-secondary"
			on:click={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>
		<button
			class="btn btn-primary"
			on:click={() => {
				isEditing = false;
				(document.getElementById('create-student-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Admit</button
		>
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<div class="alert alert-warning">Please create a session first</div>
{:else if classes.get_by_current_session().length > 0}
	{#if students_d.length > 0}
		<div class="mt-4 flex gap-2">
			<div class="w-1/2">
				<div class="bg-base-100 border-base-300 w-full flex-1 overflow-auto rounded border">
					<div class="max-h-[85vh] overflow-x-auto">
						<table class="table-pin-rows table">
							<thead>
								<tr class="bg-base-200">
									<th>{filter.class === '' ? '#' : 'Roll'}</th>
									<th>Name</th>
									<th>Class</th>
									<th>Section</th>
									<!-- <th>Roll</th> -->
								</tr>
							</thead>
							<tbody>
								{#each students_d as student, i ((student.id, i))}
									<tr
										class="{student.id === selectedStudent
											? 'bg-primary text-primary-content'
											: ''} cursor-pointer"
										on:click={() => {
											selectedStudent = student.id;
										}}
									>
										<td>{filter.class === '' ? i : student.roll}</td>
										<td>{student.name}</td>
										<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
										<td>
											{#if student.section_id}
												{sections.get(student.section_id)?.name}
											{:else}
												Base
											{/if}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>

			<div class="w-1/2">
				<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
					<h2 class="text-primary border-accent mb-3 border-b-1 pb-2 text-xl font-bold">
						Student Details
					</h2>

					{#if selectedStudent && selectedStudentData}
						<div class="text-sm">
							<div class="grid grid-cols-2 gap-4">
								<div class="space-y-3">
									<div>
										<p class="text-secondary">Name</p>
										<p class="font-medium">{selectedStudentData.name}</p>
									</div>

									<div>
										<p class="text-secondary">Gender</p>
										<p class="font-medium">{selectedStudentData.gender}</p>
									</div>

									<div>
										<p class="text-secondary">Date of Birth</p>
										<p class="font-medium">{selectedStudentData.dob}</p>
									</div>

									<div>
										<p class="text-secondary">Phone</p>
										<p class="font-medium">
											{selectedStudentData.phone || 'N/A'}
										</p>
									</div>

									<div>
										<p class="text-secondary">Admission Date</p>
										<p class="font-medium">{selectedStudentData.admission_date}</p>
									</div>

									<div>
										<p class="text-secondary">Class</p>
										<p class="font-medium">
											{classes.get(
												sessions.selected as number,
												selectedStudentData.class_id as number
											)?.name}
										</p>
									</div>

									{#if selectedStudentData.section_id !== null}
										<div>
											<p class="text-secondary">Section</p>
											<p class="font-medium">
												{sections.get(selectedStudentData.section_id)?.name}
											</p>
										</div>
									{/if}

									<div>
										<p class="text-secondary">Roll</p>
										<p class="font-medium">{selectedStudentData.roll}</p>
									</div>

									<div>
										<p class="text-secondary">Resident</p>
										<p class="font-medium">
											{selectedStudentData.is_resident ? 'Yes' : 'No'}
										</p>
									</div>

									<div>
										<p class="text-secondary">Religion</p>
										<p class="font-medium">{selectedStudentData.religion}</p>
									</div>

									<div class="flex flex-wrap gap-3 pt-2">
										<button
											class="btn btn-info btn-sm"
											on:click={() => {
												isEditing = true;
												(
													document.getElementById('create-student-modal') as HTMLDialogElement
												).show();
											}}
										>
											Edit
										</button>
										<button class="btn btn-error btn-sm" on:click={deleteStudent}> Delete</button>
									</div>
								</div>
								<div class="space-y-3">
									<img
										src={selectedStudentData.photo}
										alt="Photo of {selectedStudentData.name}"
										class="h-38 w-38 flex-shrink-0 rounded object-cover"
									/>

									{#if selectedStudentData.health_notes}
										<div>
											<p class="text-secondary">Health Notes</p>
											<div class="bg-base-200 max-h-22 overflow-y-auto rounded p-2">
												<p class="font-medium text-wrap">{selectedStudentData.health_notes}</p>
											</div>
										</div>
									{/if}
									{#if selectedStudentData.general_notes}
										<div>
											<p class="text-secondary">General Notes</p>
											<div class="bg-base-200 max-h-22 overflow-y-auto rounded p-2">
												<p class="font-medium text-wrap">{selectedStudentData.general_notes}</p>
											</div>
										</div>
									{/if}
									<div>
										<p class="text-secondary">Address</p>
										<div class="bg-base-200 max-h-22 overflow-y-auto rounded p-2">
											<p class="font-medium">{selectedStudentData.address}</p>
										</div>
									</div>
								</div>
							</div>
						</div>

						{#if guardians_s.length > 0}
							<div class="border-accent mt-4">
								<h2 class="text-primary border-accent mb-3 border-b-1 pb-2 text-xl font-bold">
									Guardians
								</h2>
								<ul class="grid max-h-48 grid-cols-2 gap-4 overflow-y-auto pr-1">
									{#each guardians_s as g, i ((g.id, i))}
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
												<p class="truncate text-sm text-gray-500">
													{g.relation ?? '—'} • {g.phone}
												</p>

												<!-- <p class="truncate text-sm text-gray-500">Smone • {g.phone}</p> -->
												<p class="truncate text-sm text-gray-500">{g.address ?? 'No address'}</p>
											</div>
										</li>
									{/each}
								</ul>
							</div>
						{/if}
					{:else}
						<p class="text-secondary alert alert-warning text-sm">
							Select a student to view details.
						</p>
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<p class="text-secondary alert alert-warning text-sm">No student has found for this query.</p>
	{/if}
{:else}
	<p class="text-secondary alert alert-warning text-sm">You haven't created any class yet.</p>
{/if}

<Filter bind:filter />
<CreateStudent bind:isEditing {selectedStudentData} />
