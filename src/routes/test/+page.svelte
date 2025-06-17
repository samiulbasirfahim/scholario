<script lang="ts">
	import { format, getDaysInMonth, startOfMonth, subMonths } from 'date-fns';

	const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

	const today = new Date();
	const months = Array.from({ length: 3 }, (_, i) => {
		const date = subMonths(today, 2 - i);
		const year = date.getFullYear();
		const month = date.getMonth();
		const firstDayOfMonth = startOfMonth(date);
		const startDay = firstDayOfMonth.getDay();
		const days = getDaysInMonth(date);

		return {
			label: format(date, 'MMMM yyyy'),
			startDay,
			days,
			year,
			month
		};
	});
</script>

<div class="flex flex-wrap">
	{#each months as m (m.label)}
		<div class="w-full p-4 md:w-auto">
			<h1 class="mb-2 text-xl font-bold">{m.label}</h1>
			<div class="grid grid-cols-7 gap-2">
				{#each daysOfWeek as day (day)}
					<span class="rounded bg-teal-500 p-2 text-center text-white">{day}</span>
				{/each}

				{#each Array(m.startDay) as _, i (i)}
					<span></span>
				{/each}

				{#each Array.from({ length: m.days }, (_, i) => i + 1) as day (day)}
					<span class="rounded bg-red-100 p-2 text-center">{day}</span>
				{/each}
			</div>
		</div>
	{/each}
</div>
