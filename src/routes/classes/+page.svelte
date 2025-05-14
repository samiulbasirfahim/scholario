<script lang="ts">
	import ClassEdit from '$lib/components/classes/ClassEdit.svelte';
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let selectedClass: number | null = $state(null);

	onMount(() => {
		classes.fetch();
		subjects.fetch();
		sections.fetch();
		classSubjects.fetch_all();
	});
</script>

<div class="navbar bg-base-100 rounded px-4">
	<div class="flex-1">
		<p class="text-sm">Classes & Subjects</p>
	</div>
	<div class="flex gap-2">
		<button
			onclick={() => {
				(document.getElementById('list-subject-modal') as HTMLDialogElement).showModal();
			}}
            class="btn btn-secondary"
		>
			<Icon icon="foundation:list" />
			Subjects
		</button>

		<button
			class="btn btn-accent"
			onclick={() => {
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Create Section
		</button>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Create Class
		</button>
	</div>
</div>

{#if classes.data.length > 0}
	<div class="border-base-content/5 bg-base-100 overflow-x-auto rounded border">
		<table class="table">
			<thead class="bg-base-200">
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
					<tr class="hover:bg-base-200">
						<th class="w-4">{i + 1}</th>
						<td colspan="2">
							{cls.name}
						</td>
						<td> 40 </td>
						<td> {sections.get_by_class(cls.id).length} </td>
						<td> {classSubjects.get(cls.id).length} </td>
						<td>
							<button
								class="btn btn-ghost btn-square btn-primary"
								onclickcapture={() => {
									(document.getElementById('class-edit-modal') as HTMLDialogElement).showModal();
									selectedClass = cls.id;
								}}
							>
								<Icon icon="lucide:edit" font-size="24" />
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
