<script lang="ts">
	import { attendanceStore } from '$lib/store/attendance.svelte';

	const { selectedStudentData, selected_month } = $props();

	import type { Attendance } from '$lib/types/attendance';
	import { getDaysInMonth, startOfMonth } from 'date-fns';

	let attendances = $state<Attendance[]>([]);

	$effect(() => {
		// console.log('Im fetching again, cause something changed', attendanceStore.reactiveCounter);
		let cnt = attendanceStore.reactiveCounter;
		(async () => {
			if (selectedStudentData && selectedStudentData.id) {
				attendances = (await attendanceStore.get(
					selectedStudentData.id,
					selected_month.year_month
				)) as Attendance[];
			}
		})();
	});

	const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

	function getInfo(day: number) {
		console.log('Getting info for ', `${selected_month}-${day}`);
		let rec = attendances.find((r) => r.date === `${selected_month.year_month}-${day}`);
		if (!rec) return { bg: 'bg-primary/20 tooltip-primary', tooltip: 'No entry.' };

		switch (rec.status) {
			case 'ABSENT':
				return { bg: 'bg-error text-error-content tooltip-error', tooltip: rec.status };
			case 'PRESENT':
				return { bg: 'bg-success text-success-content tooltip-success', tooltip: rec.status };
			default:
				return { bg: 'bg-warning text-warning-content tooltip-warning', tooltip: rec.status };
		}
	}
</script>

<div>
	<h1 class="mb-4 text-xl font-bold">{selected_month.title}</h1>
	<div class="flex">
		<div class="grid grid-cols-7 gap-2">
			{#each daysOfWeek as day (day)}
				<span class="bg-primary text-primary-content rounded p-2 text-center text-sm">{day}</span>
			{/each}

			{#each Array.from({ length: startOfMonth(selected_month.date).getDay() }, (i) => i) as _, i (i)}
				<span></span>
			{/each}

			{#each Array.from({ length: getDaysInMonth(selected_month.date) ?? 0 }, (_, i) => i + 1) as day (day)}
				<span
					class={`tooltip tooltip-top rounded p-2 text-center text-sm ${getInfo(day).bg}`}
					data-tip={getInfo(day).tooltip}
				>
					{day}
				</span>
			{/each}
		</div>
	</div>
</div>
