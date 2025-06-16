<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { attendanceStore } from '$lib/store/attendance.svelte.js';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Attendance } from '$lib/types/attendance.js';
	import type { Student } from '$lib/types/student';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	const { data } = $props();

	let selectedStudent = $state<number | null>(null);
	let selectedStudentData = $state<Student | null>();

	onMount(() => {
		selectedStudent = Number(data.selectedStudent);
	});

	$effect(() => {
		if (selectedStudent) {
			goto('?selectedStudent=' + selectedStudent, { replaceState: true });
			selectedStudentData = students.getById(selectedStudent);
		}
	});

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: ''
	});

	let students_d = $state<Student[]>([]);
	let attendanceRecords = $state([]);
	let loading = $state(false);

	$effect(() => {
		console.log(attendanceStore.reactiveCounter);
		(async () => {
			if (!selectedStudent) {
				attendanceRecords = [];
				return;
			}

			loading = true;
			attendanceRecords = await attendanceStore.get(selectedStudent);
			loading = false;
		})();
	});

	$effect(() => {
		const sessionId = sessions.selected as number;

		const classId = filter.class === '' ? undefined : Number(filter.class);
		const sectionId = filter.section === '' ? undefined : Number(filter.section);

		students.get(sessionId, classId, sectionId).then((d) => {
			students_d = d;
		});
	});

	type AttendanceStatus = 'present' | 'absent' | 'late';
	let attendance = $state.raw<Record<string, string>>({});

	function takeAttendance(id: number, status: AttendanceStatus) {
		attendance = { ...attendance, [id]: status };
	}

	$effect(() => {
		console.log(attendance);
	});

	function formatStatus(status: string) {
		if (status.startsWith('LATE-')) {
			return `Late (${status.slice(5)})`;
		}
		return status.charAt(0) + status.slice(1).toLowerCase();
	}

	async function saveAttendance() {
		const today = new Date().toISOString().slice(0, 10);

		for (const [idStr, rawStatus] of Object.entries(attendance)) {
			const student_id = Number(idStr);
			let status = rawStatus.toUpperCase();

			if (status === 'LATE') {
				const custom = prompt(
					`Late by how much for ${students.getById(student_id)?.name} (e.g., 10m or 2p)`
				);
				if (!custom || custom.trim() === '') continue;
				status = `LATE-${custom.trim()}`;
			}

			try {
				const record = await invoke<Attendance>('create_attendance', {
					student_id,
					date: today,
					status
				});

				attendanceStore.add(record);
			} catch (err) {
				console.error(`Failed to save attendance for student ${student_id}`, err);
			}
		}
	}
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
			on:change={(e) => sessions.select(Number((e.target as HTMLOptionElement).value))}
		>
			{#each sessions.data as session (session.id)}
				<option value={session.id}>{session.name}</option>
			{/each}
		</select>

		<button
			class="btn btn-secondary btn-sm"
			on:click={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>
		<button class="btn btn-secondary btn-sm" on:click={saveAttendance}>
			<Icon icon="material-symbols:save" font-size="18" />
			Save
		</button>
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
									<th class="w-4">{filter.class === '' ? '#' : 'Roll'}</th>
									<th>Name</th>
									{#if filter.class === ''}
										<th>Class</th>
									{/if}
									{#if filter.section === ''}
										<th>Section</th>
									{/if}
									<th>Status</th>
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
										{#if filter.class === ''}
											<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
										{/if}
										{#if filter.section === ''}
											<td>
												{sections.get_by_class(Number(filter.class)).length}
												{#if student.section_id}
													{sections.get(student.section_id)?.name}
												{:else}
													Base
												{/if}
											</td>
										{/if}
										<td>
											<select
												class="select bg-secondary text-secondary-content select-sm"
												on:change={(e) => {
													takeAttendance(
														student.id,
														(e.target as HTMLSelectElement).value as AttendanceStatus
													);
												}}
											>
												<option disabled selected>Pick status</option>
												<option value="present">Present</option>
												<option value="late">Late</option>
												<option value="absent">Absent</option>
											</select>
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
						Attendance History
					</h2>

					{#if !selectedStudent || !selectedStudentData}
						<div class="alert alert-warning text-secondary text-sm">
							Select a student to view their attendance history.
						</div>
					{:else if loading}
						<div class="loading loading-spinner text-secondary">Loading attendance...</div>
					{:else if attendanceRecords.length === 0}
						<div class="alert alert-info text-secondary text-sm">
							No attendance records found for {selectedStudentData.name}.
						</div>
					{:else}
						<ul class="divide-base-300 divide-y">
							{#each attendanceRecords.sort((a, b) => b.date.localeCompare(a.date)) as record}
								<li class="py-2">
									<div class="flex items-center justify-between">
										<span class="text-base-content">
											<span class="font-semibold">{record.date}</span>
										</span>
										<span class="badge badge-outline">
											{formatStatus(record.status)}
										</span>
									</div>
									{#if record.notes}
										<p class="text-secondary mt-1 text-xs">{record.notes}</p>
									{/if}
								</li>
							{/each}
						</ul>
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
