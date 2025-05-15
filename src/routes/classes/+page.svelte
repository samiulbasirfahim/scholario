<script lang="ts">
	import ClassEdit from '$lib/components/classes/ClassEdit.svelte';
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let session = $state<string>('2026');

	let selectedClass: number | null = $state(null);

	onMount(() => {
		classes.fetch();
		subjects.fetch();
		sections.fetch();
		classSubjects.fetch_all();
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm">
			<ul>
				<li>Classes</li>

				{#if session}
					<li>{session.trim().split(' ')[0]}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-4">
		<label class="bg-accent text-accent-content flex items-center rounded border-1 px-2">
			<Icon icon="carbon:prompt-session" font-size="24" />
			<select
				bind:value={session}
				class="select rounded-none border-0 bg-transparent focus:outline-none"
			>
				<option disabled>Session</option>
				<option>2024</option>
				<option>2025</option>
				<option selected>2026</option>
			</select>
		</label>
		<button
			class="btn btn-accent"
			onclick={() => {
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Section
		</button>
		<button
			class="btn btn-primary"
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
			class="btn btn-secondary"
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
<CreateSubject />
<ListSubject />
<ClassEdit {selectedClass} />
