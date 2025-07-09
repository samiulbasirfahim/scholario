<script lang="ts">
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import Icon from '@iconify/svelte';
	import LeftCol from './LeftCol.svelte';
	import RightCol from './RightCol.svelte';
	import { classes } from '$lib/store/class.svelte';

	let isEditing = $state<boolean>(false);
	let selectedClass: number | null = $state<number | null>(null);
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm font-semibold">
			<ul>
				<li>Classes</li>
				{#if sessions.selectedSession}
					<li>{sessions.selectedSession.name.trim().split(' ')[0]}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex w-full justify-end gap-2">
		<div>
			<select
				class="select select-sm select-accent border-dashed"
				bind:value={sessions.selected}
				onchange={(e) => sessions.select(Number((e.target as HTMLOptionElement).value))}
			>
				{#each sessions.data as session (session.id)}
					<option value={session.id}>{session.name}</option>
				{/each}
			</select>
		</div>

		<button
			class="btn btn-primary btn-sm"
			onclick={() =>
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal()}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Section
		</button>

		<button
			class="btn btn-secondary btn-sm"
			onclick={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Class
		</button>

		<button
			class="btn btn-accent btn-sm"
			onclick={() =>
				(document.getElementById('list-subject-modal') as HTMLDialogElement).showModal()}
		>
			<Icon icon="duo-icons:book-2" font-size="20" />
			Subjects
		</button>
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<p class="alert alert-warning text-sm">Please create a session first</p>
{:else if classes.get_by_current_session().length === 0}
	<p class="alert alert-warning text-sm">You haven't created any class yet.</p>
{:else}
	<div class="flex flex-1 flex-col gap-2 overflow-hidden xl:flex-row">
		<LeftCol bind:selectedClass />
		<RightCol bind:isEditing {selectedClass} />
	</div>
{/if}

<CreateSection class_id={selectedClass} />
<CreateSubject />
<ListSubject />
<CreateClass bind:isEditing {selectedClass} />
