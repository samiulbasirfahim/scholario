<script lang="ts">
	import { attendanceStore } from '$lib/store/attendance.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import type { Attendance } from '$lib/types/attendance';
	import { invoke } from '@tauri-apps/api/core';

	const { selectedDate, filter } = $props();

	let attendances = $state<Attendance[]>([]);

	let presents = $derived(attendances.filter((res) => res.status === 'PRESENT'));
	let absents = $derived(attendances.filter((res) => res.status === 'ABSENT'));
	let lates = $derived(attendances.filter((res) => res.status.startsWith('LATE')));
	let total_students = $state<number>(0);

	$effect(() => {
		const sessionId = sessions.selected as number;

		const classId = filter.class === '' ? undefined : Number(filter.class);
		const sectionId = filter.section === '' ? undefined : Number(filter.section);

		students.get(sessionId, classId, sectionId).then((d) => {
			total_students = d.length;
		});
	});

	$effect(() => {
		(async () => {
			console.log(attendanceStore.reactiveCounter);
			try {
				let args: { date; string; session_id: number; class_id?: number; section_id?: number } = {
					date: selectedDate
				};
				args.session_id = sessions.selected as number;
				args.class_id = filter.class === '' ? undefined : Number(filter.class);
				args.section_id = filter.section === '' ? undefined : Number(filter.section);
				attendances = await invoke<Attendance[]>('get_attendance_by_date', args);
			} catch (err) {
				console.log(err);
			}
		})();
	});

	function getStudent(id: number) {
		return students.getById(id);
	}
</script>

{#snippet stat(data: Attendance[])}
	<ul class="grid grid-cols-2 gap-2 pb-20">
		{#each data as rec (rec.id)}
			<a
				class="bg-base-300 flex cursor-pointer items-center gap-4 rounded p-2 shadow-sm"
				href={`/students?selectedStudent=${rec.student_id}`}
			>
				<div class="size-12 flex-shrink-0 overflow-hidden rounded-full">
					<img
						src={getStudent(rec.student_id)?.photo}
						alt={`Photo of ${getStudent(rec.student_id)?.name}`}
						class="h-full w-full object-cover object-center"
					/>
				</div>
				<div class="min-w-0 flex-1">
					<p class="truncate font-medium">{getStudent(rec.student_id)?.name}</p>
					<p class="truncate text-sm text-gray-500">
						<!-- {getStudent(rec.student_id)?.class_id ?? '—'} • {g.phone} -->
						{classes.get(sessions.selected, getStudent(rec.student_id)?.class_id)?.name}
						•
						{sections.get(getStudent(rec.student_id)?.section_id)?.name ?? ''}
					</p>

					<p class="truncate text-sm text-gray-500">
						{getStudent(rec.student_id)?.address ?? 'No address'}
					</p>
				</div>
			</a>
		{/each}
	</ul>
{/snippet}

<div class="flex flex-1 flex-col overflow-hidden">
	<div class="bg-base-100 flex flex-col gap-2 rounded p-4 overflow-hidden">
		<div class="flex justify-between">
			<p class="font-semibold">Total Students: {total_students}</p>
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
