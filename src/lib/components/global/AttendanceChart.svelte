<script lang="ts">
	import { onMount } from 'svelte';
	import Chart from 'chart.js/auto';

	let canvas: HTMLCanvasElement;

	let chart: Chart;

	const labels = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
	const present = [120, 130, 125, 140, 138, 128];
	const absent = [10, 5, 12, 7, 9, 6];

	onMount(() => {
		if (chart) chart.destroy(); // Cleanup if already initialized

		chart = new Chart(canvas, {
			type: 'bar',
			data: {
				labels,
				datasets: [
					{
						label: 'Present',
						data: present,
						backgroundColor: '#16a34a' // green-600
					},
					{
						label: 'Absent',
						data: absent,
						backgroundColor: '#dc2626' // red-600
					}
				]
			},
			options: {
				responsive: true,
				plugins: {
					legend: {
						position: 'top'
					},
					title: {
						display: true,
						text: 'Attendance Overview'
					}
				},
				scales: {
					y: {
						beginAtZero: true
					}
				}
			}
		});
	});
</script>

<div class="w-full">
	<canvas bind:this={canvas}></canvas>
</div>
