<script lang="ts">
	import { sessions } from '$lib/store/session.svelte';
	import { addMonths, format, getDaysInMonth, isAfter, parse, parseISO, startOfMonth } from 'date-fns';

	let availableMonths = $state([]);
	let index = $state(-1);

	let months = $state([]);

	function getMonth(i: number) {
		const date = parse(availableMonths[i], 'MMMM yyyy', new Date());
		const month = date.getMonth();
		const year = date.getFullYear();
		const firstDayOfMonth = startOfMonth(date);
		const startDay = firstDayOfMonth.getDay();
		const days = getDaysInMonth(date);

		months = [
			{
				label: format(date, 'MMMM yyyy'),
				startDay,
				days,
				year,
				month
			}
		];
	}

	function getCurrentOrLastMonthIndex() {
		const todayStr = format(new Date(), 'MMMM yyyy');
		const currentIndex = availableMonths.indexOf(todayStr);
		return currentIndex !== -1 ? currentIndex : availableMonths.length - 1;
	}

	$effect(() => {
		if (!(sessions.selectedSession?.end_date && sessions.selectedSession.end_date)) return;

		let temp = [];
		let startingDate = parseISO(sessions.selectedSession?.start_date);
		let endingDate = parseISO(sessions.selectedSession?.end_date);

		while (!isAfter(startingDate, endingDate)) {
			temp.push(format(startingDate, 'MMMM yyyy'));
			startingDate = addMonths(startingDate, 1);
		}

		availableMonths = temp;
	});

	$effect(() => {
		index = getCurrentOrLastMonthIndex();
	});

	$effect(() => {
		if (index >= 0 && availableMonths.length > 0) {
			getMonth(index);
		}
	});
</script>

{JSON.stringify(availableMonths)}
<br />
{index}
<br />
{JSON.stringify(months)}
