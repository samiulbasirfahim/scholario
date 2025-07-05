<script lang="ts">
	import { onMount } from 'svelte';
	import Calendar from './Calendar.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { addMonths, format, isAfter, parseISO } from 'date-fns';
	const { selectedStudentData, for_whom } = $props();
	interface Month {
		title: string;
		year_month: string;
		date?: Date;
	}

	let selected_month = $state<Month>({
		title: '',
		year_month: ''
	});

	let months_for_session = $state<Month[]>([]);

	onMount(() => {
		let today = new Date();
		const localISOString = new Date(
			today.getTime() - today.getTimezoneOffset() * 60000
		).toISOString();
		selected_month = {
			title: format(today, 'MMMM yyyy'),
			year_month: localISOString.slice(0, 7),
			date: today
		};
	});

	$effect(() => {
		if (!sessions.selectedSession) return;

		let start_date = parseISO(sessions.selectedSession.start_date);
		let end_date = parseISO(sessions.selectedSession.end_date);

		let tmp: Month[] = [];

		if (!start_date || !end_date) return;

		while (!isAfter(start_date, end_date)) {
			tmp.push({
				title: format(start_date, 'MMMM yyyy'),
				year_month: start_date.toISOString().slice(0, 7),
				date: start_date
			});
			start_date = addMonths(start_date, 1);
		}

		months_for_session = tmp;
	});
</script>

<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
	<h2 class="text-primary border-accent mb-3 border-b-1 pb-2 text-xl font-bold">
		Attendance History
	</h2>

	{#if !selectedStudentData}
		<div class="alert alert-warning text-sm">
			Select a student to view their attendance history.
		</div>
	{:else}
		<div class="flex justify-between">
			<Calendar {selectedStudentData} {selected_month} {for_whom} />
			<select
				bind:value={selected_month.year_month}
				class="select select-accent mb-4 max-w-40"
				onchange={() => {
					let date = months_for_session.find(
						(m) => m.year_month === selected_month.year_month
					)?.date;
					if (!date) return;
					let ym = selected_month.year_month;
					selected_month = {
						date,
						year_month: ym,
						title: format(date as Date, 'MMMM yyyy')
					};
				}}
			>
				{#each months_for_session as month (month)}
					<option value={month.year_month}>{month.title}</option>
				{/each}
			</select>
		</div>
	{/if}
</div>
