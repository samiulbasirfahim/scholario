<script lang="ts">
	import Navbar from '$lib/components/global/Navbar.svelte';
	import CreateStudent from '$lib/components/students/CreateStudent.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Student } from '$lib/types/student';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import StudentTable from './StudentTable.svelte';
	import StudentDetails from './StudentDetails.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { toast } from '$lib/store/toast.svelte';
	import { Confirm } from '$lib/utility/Confirm';
	const { data } = $props();

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

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: ''
	});

	let isEditing = $state(false);

	let students_d = $state<Student[]>([]);

	$effect(() => {
		const sessionId = sessions.selected as number;

		const classId = filter.class === '' ? undefined : Number(filter.class);
		const sectionId = filter.section === '' ? undefined : Number(filter.section);

		students.get(sessionId, classId, sectionId).then((d) => {
			students_d = d;
		});
	});

	const deleteStudent = async (student_id: number) => {

		try {
			if (student_id) {
				await invoke('delete_student', {
					id: student_id,
					session_id: sessions.selected as number
				});
				students.remove(student_id);
				if (selectedStudent === student_id) selectedStudent = null;
				toast.set({ message: 'Student deleted', type: 'success' });
			}
		} catch (err) {
			console.log(err);
			toast.set({ message: 'Failed to delete student', type: 'error' });
		}
	};
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm font-semibold">
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
		<div class="isolate">
			<input
				type="text"
				placeholder="Search"
				class="input input-sm input-bordered w-48 transform-gpu transition-all ease-linear focus:w-64 focus:outline-none"
			/>
		</div>

		<select
			class="select select-sm select-accent"
			bind:value={sessions.selected}
			onchange={(e) => sessions.select(Number((e.target as HTMLOptionElement).value))}
		>
			{#each sessions.data as session (session.id)}
				<option value={session.id}>{session.name}</option>
			{/each}
		</select>

		<button
			class="btn btn-secondary btn-sm"
			onclick={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>
		<button
			class="btn btn-primary btn-sm"
			onclick={() => {
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
		<div class="flex flex-1 gap-2 overflow-hidden">
			<StudentTable {students_d} bind:selectedStudent {deleteStudent} />
			<StudentDetails {selectedStudent} {selectedStudentData} bind:isEditing {deleteStudent} />
		</div>
	{/if}
{:else}
	<p class="alert alert-warning text-sm">You haven't created any class yet.</p>
{/if}

<Filter bind:filter />
<CreateStudent bind:isEditing {selectedStudentData} />
