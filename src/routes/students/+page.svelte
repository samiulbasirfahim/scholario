<script lang="ts">
	import Navbar from '$lib/components/global/Navbar.svelte';
	import CreateStudent from '$lib/components/students/CreateStudent.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Student } from '$lib/types/student';
	import Icon from '@iconify/svelte';

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: ''
	});

	let isEditing = $state(false);

	let students_d = $state<Student[]>([]);

	let selectedStudent = $state<number>(-1);
	let selectedStudentData = $derived(students.getById(selectedStudent));

	let deleteStudent = () => {};

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
				<div class="bg-base-100 border-base-300 w-full flex-1 overflow-auto rounded border p-4">
					<div class="max-h-[85vh] overflow-x-auto">
						<table class="table-pin-rows table">
							<thead>
								<tr>
									<th>#</th>
									<th>Name</th>
									<th>Class</th>
									<th>Section</th>
									<th>Roll</th>
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
										<th>{i + 1}</th>
										<td>{student.name}</td>
										<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
										<td>
											{#if student.section_id >= 0}
												{sections.get(student.section_id)?.name}
											{:else}
												"0"
											{/if}
										</td>
										<td>40</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>

			<div class="w-1/2">
				<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
					<h2 class="text-primary mb-3 text-xl font-bold">Student Details</h2>

					{#if selectedStudent >= 0 && selectedStudentData}
						<div class="text-sm">
							<div class="grid grid-cols-2 gap-4">
								<div class="space-y-2">
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

								<div class="space-y-2">
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
								</div>
							</div>

							<div class="mt-2 space-y-2 border-t-1 pt-2">
								{#if selectedStudentData.health_notes}
									<div>
										<p class="text-secondary">Health Notes</p>
										<p class="font-medium">{selectedStudentData.health_notes}</p>
									</div>
								{/if}
								{#if selectedStudentData.general_notes}
									<div>
										<p class="text-secondary">General Notes</p>
										<p class="font-medium">{selectedStudentData.general_notes}</p>
									</div>
								{/if}
								<div>
									<p class="text-secondary">Address</p>
									<p class="font-medium">{selectedStudentData.address}</p>
								</div>
							</div>
						</div>
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
