<script lang="ts">
	import Icon from '@iconify/svelte';

	let activeTab = 'overview';

	let stats = [
		{ label: 'Total Students', value: 428, icon: 'mdi:account-school' },
		{ label: 'Teachers', value: 32, icon: 'mdi:teach' },
		{ label: 'Staff', value: 18, icon: 'mdi:account-tie' },
		{ label: 'Attendance Today', value: '392 / 428', icon: 'mdi:clipboard-check' },
		{ label: 'Monthly Revenue', value: '৳1,54,000', icon: 'mdi:cash' }
	];

	let upcomingExams = [
		{ subject: 'Mathematics', class: '10-A', date: 'July 12' },
		{ subject: 'English', class: '9-B', date: 'July 14' },
		{ subject: 'Physics', class: '10-C', date: 'July 16' }
	];

	let pendingFees = {
		students: 27,
		amount: '৳34,200'
	};

	let recentActivities = [
		{ time: '9:05 AM', message: 'Class 8 attendance taken' },
		{ time: '9:20 AM', message: '2 new students admitted' },
		{ time: '10:00 AM', message: 'Math exam schedule updated' },
		{ time: '11:30 AM', message: 'Teacher meeting scheduled' }
	];
</script>

<div class="bg-base-200 flex-1 p-6">
	<!-- Header -->
	<div class="mb-4 flex items-center justify-between">
		<h1 class="text-xl font-bold">📊 School Dashboard</h1>
		<div class="tabs tabs-boxed">
			<button
				class="tab"
				class:selected={activeTab === 'overview'}
				on:click={() => (activeTab = 'overview')}
			>
				Overview
			</button>
			<button
				class="tab"
				class:selected={activeTab === 'fees'}
				on:click={() => (activeTab = 'fees')}
			>
				Fees
			</button>
			<button
				class="tab"
				class:selected={activeTab === 'exams'}
				on:click={() => (activeTab = 'exams')}
			>
				Exams
			</button>
			<button
				class="tab"
				class:selected={activeTab === 'activity'}
				on:click={() => (activeTab = 'activity')}
			>
				Activity
			</button>
		</div>
	</div>

	<!-- Tab: Overview -->
	{#if activeTab === 'overview'}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5">
			{#each stats as { label, value, icon }, i (i)}
				<div class="bg-base-100 rounded p-4 shadow-sm">
					<div class="flex items-center gap-3">
						<Icon {icon} class="text-primary text-2xl" />
						<div>
							<p class="text-sm text-gray-500">{label}</p>
							<p class="text-lg font-semibold">{value}</p>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Tab: Fees -->
	{#if activeTab === 'fees'}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">Pending Fees</h2>
				<p class="text-sm text-gray-600">{pendingFees.students} students</p>
				<p class="text-error text-2xl font-bold">{pendingFees.amount}</p>
			</div>
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">Fee Collection Trends</h2>
				<div class="flex h-24 items-center justify-center text-gray-400">[Graph Placeholder]</div>
			</div>
		</div>
	{/if}

	<!-- Tab: Exams -->
	{#if activeTab === 'exams'}
		<div class="bg-base-100 max-w-xl rounded p-4 shadow-sm">
			<h2 class="mb-2 text-lg font-semibold">Upcoming Exams</h2>
			<ul class="space-y-2 text-sm text-gray-700">
				{#each upcomingExams as exam, i (i)}
					<li class="flex justify-between">
						<span>{exam.subject} <span class="text-xs text-gray-400">({exam.class})</span></span>
						<span class="text-xs text-gray-500">{exam.date}</span>
					</li>
				{/each}
			</ul>
		</div>
	{/if}

	<!-- Tab: Activity -->
	{#if activeTab === 'activity'}
		<div class="bg-base-100 max-w-xl rounded p-4 shadow-sm">
			<h2 class="mb-2 text-lg font-semibold">Recent Activities</h2>
			<ul class="space-y-2 text-sm text-gray-700">
				{#each recentActivities as act, i (i)}
					<li class="flex justify-between">
						<span>{act.message}</span>
						<span class="text-xs text-gray-400">{act.time}</span>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>

<style>
	.tab[selected],
	.tab[class~='selected'] {
		background-color: hsl(var(--p));
		color: white;
	}
</style>
