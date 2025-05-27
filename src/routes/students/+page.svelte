<script lang="ts">
	import Navbar from '$lib/components/global/Navbar.svelte';
	import CreateStudent from '$lib/components/students/CreateStudent.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let filter = $state({
		class: '',
		section: '',
		fee: '',
		roll: '',
		session: '2026'
	});

	onMount(async () => {
		await sessions.fetch();
		sessions.select(sessions.data[sessions.data?.length - 1]?.id);
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm">
			<ul>
				<li>Students</li>

				{#if filter.session}
					<li>{filter.session.trim().split(' ')[0]}</li>
				{/if}
				{#if filter.class}
					<li>{classes.get(sessions.selected as number, Number(filter.class))?.name}</li>
				{/if}
				{#if filter.section}
					<li>{sections.get(Number(filter.section))?.name}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-2">
		<input
			type="text"
			placeholder="Search"
			class="input input-bordered w-48 transition-all ease-linear focus:w-64 focus:outline-none"
		/>

		<label class="bg-accent text-accent-content flex items-center rounded border-1 px-2">
			<Icon icon="carbon:prompt-session" font-size="24" />
			<select
				class="select rounded-none border-0 bg-transparent focus:outline-none"
				value={sessions.selected}
				onchange={(e) => sessions.select(Number(e.target.value))}
			>
				{#each sessions.data as session (session.id)}
					<option value={session.id}>{session.name}</option>
				{/each}
			</select>
		</label>

		<button
			class="btn btn-secondary"
			onclick={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-student-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Admit</button
		>
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<div class="alert alert-warning">Please create a session first</div>
{/if}
<Filter bind:filter />
<CreateStudent />
