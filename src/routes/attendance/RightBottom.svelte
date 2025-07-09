<script lang="ts">
	import { attendanceStore, staffAttendanceStore } from '$lib/store/attendance.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import { staff } from '$lib/store/staff.svelte';
	import { invoke } from '@tauri-apps/api/core';

	const { selectedDate, filter, for_whom } = $props();

	let attendances = $state<Attendance[]>([]);

	let presents = $derived(attendances.filter((res) => res.status === 'PRESENT'));
	let absents = $derived(attendances.filter((res) => res.status === 'ABSENT'));
	let lates = $derived(attendances.filter((res) => res.status.startsWith('LATE')));
	let total = $state<number>(0);

	$effect(() => {
		const sessionId = sessions.selected as number;

		if (for_whom === 'STUDENT') {
			const classId = filter.class === '' ? undefined : Number(filter.class);
			const sectionId = filter.section === '' ? undefined : Number(filter.section);
			students.get(sessionId, classId, sectionId).then((d) => {
				total = d.length;
			});
		} else {
			staff.get().then((d) => {
				total = d.length;
			});
		}
	});

	$effect(() => {
		console.log(staffAttendanceStore.reactiveCounter);
		console.log(attendanceStore.reactiveCounter);
		(async () => {
			try {
				if (for_whom === 'STAFF') {
					attendances = await invoke<Attendance[]>('get_staff_attendance_by_date', {
						date: selectedDate
					});
				} else {
					const session_id = sessions.selected as number;
					const class_id = filter.class === '' ? undefined : Number(filter.class);
					const section_id = filter.section === '' ? undefined : Number(filter.section);
					attendances = await invoke<Attendance[]>('get_attendance_by_date', {
						date: selectedDate,
						session_id,
						class_id,
						section_id
					});
				}
			} catch (err) {
				console.log(err);
			}
		})();
	});

	import type { Attendance as StudentAttendance } from '$lib/types/attendance';
	import type { AttendanceStaff } from '$lib/types/attendance';

	function isStudent(att: StudentAttendance | AttendanceStaff): att is StudentAttendance {
		return for_whom === 'STUDENT';
	}

	function isStaff(att: StudentAttendance | AttendanceStaff): att is AttendanceStaff {
		return for_whom === 'STAFF';
	}

	function getInfo(att: StudentAttendance | AttendanceStaff) {
		if (isStudent(att)) {
			return students.getById(att.student_id);
		} else if (isStaff(att)) {
			return staff.getById(att.staff_id);
		}
		return null;
	}

	function getHref(att: StudentAttendance | AttendanceStaff): string {
		if (isStudent(att)) {
			return `/students?selectedStudent=${att.student_id}`;
		} else if (isStaff(att)) {
			const s = staff.getById(att.staff_id);
			if (!s) return '#';
			return s.is_teacher
				? `/teachers?selectedStaff=${att.staff_id}`
				: `/staffs?selectedStaff=${att.staff_id}`;
		}
		return '#';
	}
</script>

{#snippet stat(data: StudentAttendance[])}
	<ul class="grid grid-cols-2 gap-2 pb-20">
		{#each data as rec (rec.id)}
			{@const info = getInfo(rec)}
			<a
				class="bg-base-300 flex cursor-pointer items-center gap-4 rounded p-2 shadow-sm"
				href={getHref(rec)}
			>
				<div class="size-12 flex-shrink-0 overflow-hidden rounded-full">
					<img
						src={info?.photo}
						alt={`Photo of ${info?.name}`}
						class="h-full w-full object-cover object-center"
					/>
				</div>
				<div class="min-w-0 flex-1">
					<p class="truncate font-semibold">{info?.name}</p>

					{#if for_whom === 'STUDENT'}
						{@const section_ = sections.get(info?.section_id)}
						<p class="truncate text-sm opacity-80">
							{classes.get(sessions.selected, info?.class_id)?.name}
							{#if section_}
								• {section_.name}
							{/if}
						</p>
					{:else}
						<p class="truncate text-sm opacity-80">{info?.role}</p>
					{/if}

					<p class="truncate text-sm opacity-80">
						{info?.address ?? 'No address'}
					</p>
				</div>
			</a>
		{/each}
	</ul>
{/snippet}

<div class="flex flex-1 flex-col overflow-hidden">
	<div class="bg-base-100 flex flex-col gap-2 overflow-hidden rounded p-4">
		<div class="flex justify-between">
			<p class="font-semibold">Total {for_whom === 'STAFF' ? 'Staff' : 'Students'}: {total}</p>
			<p class="font-semibold">Date: {selectedDate}</p>
		</div>

		<div class="tabs tabs-border flex-1 overflow-hidden">
			<input
				type="radio"
				name="attendance"
				class="tab"
				aria-label="Presents - {presents.length}"
				checked
			/>
			<div class="tab-content mt-4 box-border overflow-auto">
				{@render stat(presents)}
			</div>

			<input type="radio" name="attendance" class="tab" aria-label="Lates - {lates.length}" />
			<div class="tab-content mt-4 overflow-auto">
				{@render stat(lates)}
			</div>

			<input type="radio" name="attendance" class="tab" aria-label="Absents - {absents.length}" />
			<div class="tab-content mt-4 overflow-auto">
				{@render stat(absents)}
			</div>
		</div>
	</div>
</div>
