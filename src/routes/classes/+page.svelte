<script lang="ts">
	import ClassEdit from '$lib/components/classes/ClassEdit.svelte';
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import type { Session } from '$lib/types/session';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let selectedClass: number | null = $state(null);

	let selectedSession: number | null = $state<number>(null);

	onMount(() => {
		sessions.fetch();
	});
	$effect(() => {
		selectedSession = sessions.data[sessions.data.length - 1]?.id;
	});

	$effect(() => {
		classes.fetch(selectedSession as number);
	});

	$effect(() => {
		classSubjects.fetch(selectedClass as number);
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm">
			<ul>
				<li>Classes</li>

				{#if selectedSession}
					<li>{sessions.get(selectedSession)?.name.trim().split(' ')[0]}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-4">
		<label class="bg-accent text-accent-content flex items-center rounded border-1 px-2">
			<Icon icon="carbon:prompt-session" font-size="24" />
			<select
				bind:value={selectedSession}
				class="select rounded-none border-0 bg-transparent focus:outline-none"
			>
				{#each sessions.data as session (session.id)}
					<option value={session.id}>{session.name}</option>
				{/each}
			</select>
		</label>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Section
		</button>
		<button
			class="btn btn-secondary"
			onclick={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Class
		</button>

		<button
			onclick={() => {
				(document.getElementById('list-subject-modal') as HTMLDialogElement).showModal();
			}}
			class="btn btn-info"
		>
			<Icon icon="duo-icons:book-2" font-size="20" />
			Subjects
		</button>
	</div>
</Navbar>

{#if classes.data.length > 0}
	<div class="border-base-content/5 bg-base-100 overflow-x-auto rounded border">
		<table class="table-md table">
			<thead class="bg-secondary text-secondary-content">
				<tr>
					<th></th>
					<th>Name</th>
					<th></th>
					<th>Students</th>
					<th>Sections</th>
					<th>Subjects</th>
					<th></th>
				</tr>
			</thead>
			<tbody>
				{#each classes.data as cls, i (i)}
					<tr>
						<th class="w-2">{i + 1}</th>
						<td colspan="2">
							{cls.name}
						</td>
						<td> 40 </td>
						<td> {sections.get_by_class(cls.id).length} </td>
						<td> {classSubjects.get(cls.id).length} </td>
						<td>
							<button
								class="btn btn-primary btn-sm"
								onclickcapture={() => {
									(document.getElementById('class-edit-modal') as HTMLDialogElement).showModal();
									selectedClass = cls.id;
								}}
							>
								Edit
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<p>No classes yet. Click 'Create Class' to get started!</p>
{/if}

<CreateClass />
<CreateSection />
<CreateSubject {selectedSession} />
<ListSubject />
<ClassEdit {selectedClass} />
