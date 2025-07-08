<script lang="ts">
	import Icon from '@iconify/svelte';

	let activeTab = 'overview';

	let stats = [
		{ label: 'Total Students', value: 428, icon: 'mdi:account-school' },
		{ label: 'Teachers', value: 32, icon: 'mdi:teach' },
		{ label: 'Staff', value: 18, icon: 'mdi:account-tie' },
		{ label: "Today's Attendance", value: '392 / 428', icon: 'mdi:clipboard-check' },
		{ label: 'Monthly Revenue', value: '৳1,54,000', icon: 'mdi:cash' }
	];

	let classStrength = [
		{ name: 'Class 10', students: 56 },
		{ name: 'Class 9', students: 61 },
		{ name: 'Class 8', students: 73 },
		{ name: 'Class 7', students: 55 },
		{ name: 'Class 6', students: 62 }
	];

	let recentAdmissions = [
		{ name: 'Shuvo Rahman', class: '6-A', date: 'Jul 6' },
		{ name: 'Tania Aktar', class: '8-B', date: 'Jul 5' },
		{ name: 'Mahin Islam', class: '9-C', date: 'Jul 3' }
	];

	let upcomingEvents = [
		{ title: 'Science Fair', date: 'July 18' },
		{ title: 'Parents Meeting', date: 'July 21' },
		{ title: 'Eid Holiday', date: 'Aug 10-15' }
	];

	let notices = [
		{ title: 'Exam Syllabus Uploaded', icon: 'mdi:file-document-outline' },
		{ title: 'Holiday on July 10', icon: 'mdi:calendar-alert' },
		{ title: 'Staff meeting at 1PM', icon: 'mdi:account-group-outline' }
	];

	let systemStatus = {
		backup: 'Last backup: 4 hours ago',
		database: 'Healthy',
		server: 'Online'
	};
</script>

<div class="bg-base-200 flex-1 p-6">
	<!-- Header & Tabs -->
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
				class:selected={activeTab === 'academic'}
				on:click={() => (activeTab = 'academic')}
			>
				Academic
			</button>
			<button
				class="tab"
				class:selected={activeTab === 'finance'}
				on:click={() => (activeTab = 'finance')}
			>
				Finance
			</button>
			<button
				class="tab"
				class:selected={activeTab === 'system'}
				on:click={() => (activeTab = 'system')}
			>
				System
			</button>
		</div>
	</div>

	<!-- Overview Tab -->
	{#if activeTab === 'overview'}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-5">
			{#each stats as s, i (i)}
				<div class="bg-base-100 rounded p-4 shadow-sm">
					<div class="flex items-center gap-3">
						<Icon icon={s.icon} class="text-primary text-2xl" />
						<div>
							<p class="text-sm text-gray-500">{s.label}</p>
							<p class="text-lg font-semibold">{s.value}</p>
						</div>
					</div>
				</div>
			{/each}
		</div>

		<div class="mt-6 grid grid-cols-1 gap-4 xl:grid-cols-3">
			<!-- Notices -->
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">📌 Notices</h2>
				<ul class="space-y-2 text-sm text-gray-700">
					{#each notices as n, i (i)}
						<li class="flex items-center gap-3">
							<Icon icon={n.icon} class="text-primary" />
							<span>{n.title}</span>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Upcoming Events -->
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">🎯 Upcoming Events</h2>
				<ul class="space-y-2 text-sm text-gray-700">
					{#each upcomingEvents as e, i (i)}
						<li class="flex justify-between">
							<span>{e.title}</span>
							<span class="text-xs text-gray-400">{e.date}</span>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Recent Admissions -->
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">🧾 Recent Admissions</h2>
				<ul class="space-y-2 text-sm text-gray-700">
					{#each recentAdmissions as s, i (i)}
						<li class="flex justify-between">
							<span>{s.name} <span class="text-xs text-gray-400">({s.class})</span></span>
							<span class="text-xs text-gray-500">{s.date}</span>
						</li>
					{/each}
				</ul>
			</div>
		</div>
	{/if}

	<!-- Academic Tab -->
	{#if activeTab === 'academic'}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each classStrength as c, i (i)}
				<div class="bg-base-100 rounded p-4 shadow-sm">
					<p class="text-sm text-gray-500">{c.name}</p>
					<p class="text-2xl font-bold">{c.students}</p>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Finance Tab -->
	{#if activeTab === 'finance'}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">📥 Revenue Chart</h2>
				<div class="flex h-32 items-center justify-center text-gray-400">[Chart Placeholder]</div>
			</div>
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<h2 class="mb-2 text-lg font-semibold">📤 Expenses Chart</h2>
				<div class="flex h-32 items-center justify-center text-gray-400">[Chart Placeholder]</div>
			</div>
		</div>
	{/if}

	<!-- System Tab -->
	{#if activeTab === 'system'}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<p class="text-sm text-gray-500">📁 {systemStatus.backup}</p>
			</div>
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<p class="text-sm text-gray-500">
					🗃️ Database: <span class="text-success font-semibold">{systemStatus.database}</span>
				</p>
			</div>
			<div class="bg-base-100 rounded p-4 shadow-sm">
				<p class="text-sm text-gray-500">
					🌐 Server Status: <span class="text-success font-semibold">{systemStatus.server}</span>
				</p>
			</div>
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
