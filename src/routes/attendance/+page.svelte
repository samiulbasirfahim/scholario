<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Student } from '$lib/types/student.js';
	import Icon from '@iconify/svelte';
	import { isAfter, isValid, parseISO } from 'date-fns';
	import { onMount } from 'svelte';
	import LeftRaw from './LeftRaw.svelte';
	import RightRaw from './RightRaw.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Attendance } from '$lib/types/attendance';
	import { attendanceStore } from '$lib/store/attendance.svelte';
	import RightBottom from './RightBottom.svelte';
	const { data } = $props();

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
		const isExceeded = isAfter(currentDate, endDate);
		return isExceeded;
	}

	let students_d = $state<Student[]>([]);
	let selectedStudent = $state<number | null>(null);
	let selectedStudentData = $state<Student | null>();
	let selectedDate = $state();

	$effect(() => {
		const sessionId = sessions.selected as number;

		const classId = filter.class === '' ? undefined : Number(filter.class);
		const sectionId = filter.section === '' ? undefined : Number(filter.section);

		students.get(sessionId, classId, sectionId).then((d) => {
			students_d = d;
		});
	});

	let attendance = $state.raw<Record<string, string>>({});

	async function saveAttendance() {
		// let today: Date | string = new Date();
		// const localISOString = new Date(
		// 	today.getTime() - today.getTimezoneOffset() * 60000
		// ).toISOString();
		// today = localISOString.slice(0, 10);

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
					date: selectedDate,
					status
				});

				attendanceStore.add(record);
			} catch (err) {
				console.error(`Failed to save attendance for student ${student_id}`, err);
			}
		}
	}

	onMount(() => {
		let today: Date | string = new Date();
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

		<div class="input input-sm">
			<div class="flex items-center justify-center gap-2">
				<Icon icon="lets-icons:date-fill" font-size="18" />
				<input
					type="date"
					min={sessions.selectedSession?.start_date}
					max={sessions.selectedSession?.end_date}
					bind:value={selectedDate}
					onblur={(e) => {
						const value = (e.target as HTMLDataElement).value;
						if (!(sessions.selectedSession?.start_date && sessions.selectedSession?.end_date))
							return;
						if (
							value < sessions.selectedSession?.start_date ||
							value > sessions.selectedSession?.end_date
						) {
							let today: Date | string = new Date();
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

		<button
			class="btn btn-secondary btn-sm"
			onclick={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>
		{#if !hasDateExceededEndDate()}
			<button class="btn btn-secondary btn-sm" onclick={saveAttendance}>
				<Icon icon="material-symbols:save" font-size="18" />
				Save
			</button>
		{/if}
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<p class="alert alert-warning text-sm">Please create a session first</p>
{:else if classes.get_by_current_session().length === 0}
	<p class="alert alert-warning text-sm">You haven't created any class yet.</p>
{:else if students_d.length === 0}
	<p class="alert alert-warning text-sm">You haven't created any student yet.</p>
{:else}
	<div class="mt-4 flex flex-col gap-2 xl:flex-row">
		<LeftRaw
			{filter}
			bind:selectedStudent
			{hasDateExceededEndDate}
			bind:students_d
			bind:attendance
		/>
		<div class="flex w-full flex-col gap-4 xl:w-1/2">
			<RightRaw {selectedStudentData} />
			<RightBottom {selectedDate} {filter} />
		</div>
	</div>
{/if}

<Filter bind:filter />
