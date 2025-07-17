<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import { staff } from '$lib/store/staff.svelte';
	import type { Student } from '$lib/types/student';
	import type { Staff } from '$lib/types/staff';
	import Icon from '@iconify/svelte';
	import { isAfter, isValid, parseISO } from 'date-fns';
	import { onMount } from 'svelte';
	import LeftRaw from './LeftRaw.svelte';
	import RightRaw from './RightRaw.svelte';
	import RightBottom from './RightBottom.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Attendance } from '$lib/types/attendance';
	import { attendanceStore } from '$lib/store/attendance.svelte';
	import { staffAttendanceStore } from '$lib/store/attendance.svelte';
	import { page } from '$app/state';

	onMount(() => {
		selectedStudent = Number(page.url.searchParams.get('selectedStudent'));
	});

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: ''
	});

	function hasDateExceededEndDate() {
		const endDateStr = sessions.selectedSession?.end_date;

		if (!endDateStr || endDateStr.trim() === '') {
			return true;
		}

		const endDate = parseISO(endDateStr);
		if (!isValid(endDate)) {
			return true;
		}

		const currentDate = new Date();
		return isAfter(currentDate, endDate);
	}

	let students_d = $state<Student[]>([]);
	let staff_d = $state<Staff[]>([]);
	let selectedStudent = $state<number | null>(null);
	let selectedStudentData = $state<Student | Staff | null>(null);
	let selectedDate = $state<string>('');

	let for_whom = $state<'STUDENT' | 'STAFF'>('STUDENT');

	// Update selectedStudentData whenever selectedStudent or for_whom changes
	$effect(() => {
		if (selectedStudent) {
			selectedStudentData =
				for_whom === 'STUDENT' ? students.getById(selectedStudent) : staff.getById(selectedStudent);
			goto(`?selectedStudent=${selectedStudent}`, { replaceState: true });
		} else {
			selectedStudentData = null;
		}
	});

	// Load students or staff list according to for_whom
	$effect(() => {
		if (for_whom === 'STUDENT') {
			const sessionId = sessions.selected as number;
			const classId = filter.class === '' ? undefined : Number(filter.class);
			const sectionId = filter.section === '' ? undefined : Number(filter.section);

			students.get(sessionId, classId, sectionId).then((d) => {
				students_d = d;
			});
		} else {
			staff.get().then((d) => {
				staff_d = d;
			});
		}
	});

	let attendance = $state.raw<Record<string, string>>({});

	async function saveAttendance() {
		for (const [idStr, rawStatus] of Object.entries(attendance)) {
			const id = Number(idStr);
			let status = rawStatus.toUpperCase();

			if (status === 'LATE') {
				const name = for_whom === 'STUDENT' ? students.getById(id)?.name : staff.getById(id)?.name;
				const custom = prompt(`Late by how much for ${name} (e.g., 10m or 2p)`);
				if (!custom || custom.trim() === '') continue;
				status = `LATE-${custom.trim()}`;
			}

			try {
				if (for_whom === 'STUDENT') {
					const record = await invoke<Attendance>('create_attendance', {
						student_id: id,
						date: selectedDate,
						status
					});
					attendanceStore.add(record);
				} else {
					const record = await invoke<Attendance>('create_attendance_staff', {
						staff_id: id,
						date: selectedDate,
						status
					});
					staffAttendanceStore.add(record);
				}
			} catch (err) {
				console.error(
					`Failed to save attendance for ${for_whom === 'STUDENT' ? 'student' : 'staff'} ${id}`,
					err
				);
			}
		}
	}

	// Fetch attendance for selected student/staff and date
	$effect(() => {
		if (selectedStudent && selectedDate) {
			if (for_whom === 'STUDENT') {
				attendanceStore.fetch(selectedStudent, selectedDate.slice(0, 4), selectedDate.slice(5, 7));
			} else {
				staffAttendanceStore.fetch(
					selectedStudent,
					selectedDate.slice(0, 4),
					selectedDate.slice(5, 7)
				);
			}
		}
	});

	onMount(() => {
		let today = new Date();
		const localISOString = new Date(
			today.getTime() - today.getTimezoneOffset() * 60000
		).toISOString();
		selectedDate = localISOString.slice(0, 10);
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm font-semibold">
			<ul>
				<li>{for_whom === 'STUDENT' ? 'Students' : 'Staff'}</li>
				{#if sessions.selectedSession}
					<li>{sessions.selectedSession.name.trim().split(' ')[0]}</li>
				{/if}
				{#if filter.class && for_whom === 'STUDENT'}
					<li>{classes.get(sessions.selected as number, Number(filter.class))?.name}</li>
				{/if}
				{#if filter.section && for_whom === 'STUDENT'}
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

		<div class="input input-sm">
			<div class="flex items-center justify-center gap-2">
				<Icon icon="lets-icons:date-fill" font-size="18" />
				<input
					type="date"
					min={sessions.selectedSession?.start_date}
					max={sessions.selectedSession?.end_date}
					bind:value={selectedDate}
					onblur={(e) => {
						const value = (e.target as HTMLInputElement).value;
						if (!(sessions.selectedSession?.start_date && sessions.selectedSession?.end_date))
							return;

						if (
							value < sessions.selectedSession?.start_date ||
							value > sessions.selectedSession?.end_date
						) {
							let today = new Date();
							const localISOString = new Date(
								today.getTime() - today.getTimezoneOffset() * 60000
							).toISOString();
							selectedDate = localISOString.slice(0, 10);
							alert('Selected date is out of range.');
						}
					}}
				/>
			</div>
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

		<select class="select select-sm select-accent" bind:value={for_whom}>
			<option value="STUDENT">Student</option>
			<option value="STAFF">Staff</option>
		</select>

		<button
			class="btn btn-secondary btn-sm"
			onclick={() => (document.getElementById('filter-modal') as HTMLDialogElement).showModal()}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>

		{#if !hasDateExceededEndDate()}
			<button class="btn btn-primary btn-sm" onclick={saveAttendance}>
				<Icon icon="material-symbols:save" font-size="18" />
				Save
			</button>
		{/if}
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<p class="alert alert-warning text-sm">Please create a session first</p>
{:else if for_whom === 'STUDENT' && classes.get_by_current_session().length === 0}
	<p class="alert alert-warning text-sm">You haven't created any class yet.</p>
{:else if for_whom === 'STUDENT' && students_d.length === 0}
	<p class="alert alert-warning text-sm">You haven't created any student yet.</p>
{:else if for_whom === 'STAFF' && staff_d.length === 0}
	<p class="alert alert-warning text-sm">You haven't created any staff yet.</p>
{:else}
	<div class="flex flex-1 flex-col gap-2 overflow-hidden xl:flex-row">
		<LeftRaw
			{filter}
			bind:selectedStudent
			{hasDateExceededEndDate}
			bind:students_d
			bind:staff_d
			bind:attendance
			{for_whom}
		/>

		<div class="flex flex-1 flex-col gap-2 rounded">
			<RightRaw {selectedStudentData} {for_whom} />
			<RightBottom {selectedDate} {filter} {for_whom} />
		</div>
	</div>
{/if}

<Filter bind:filter />
