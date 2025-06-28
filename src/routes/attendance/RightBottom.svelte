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

{#snippet stat(data: Attendance[], title: string)}
	{#if data.length > 0}
		<div class="collapse-plus bg-base-200 collapse mb-2">
			<input type="radio" name="attendance-stats" />
			<div class="collapse-title flex items-center p-4 font-semibold">
				<p>Total {title}: {data.length}</p>
			</div>
			<div class="collapse-content">
				<ul class=" grid max-h-40 grid-cols-2 gap-2 overflow-y-scroll text-sm">
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
			</div>
		</div>
	{:else}
		<div class="bg-base-200 mb-2 flex items-center rounded p-4 font-semibold">
			<p>Total {title}: {data.length}</p>
		</div>
	{/if}
{/snippet}

<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
	<h2 class="text-primary border-accent mb-3 border-b-1 pb-2 text-xl font-bold">{selectedDate}</h2>
	<p class="mb-4 font-semibold">Total Students: {total_students}</p>

	{@render stat(presents, 'Presents')}
	{@render stat(lates, 'Lates')}
	{@render stat(absents, 'Absents')}
</div>
