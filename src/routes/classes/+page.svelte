<script lang="ts">
	import ClassEdit from '$lib/components/classes/ClassEdit.svelte';
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { classes, classSubjects, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import Icon from '@iconify/svelte';

	let selectedClass = $state<number | null>(null);
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm">
			<ul>
				<li>Classes</li>

				{#if sessions.selectedSession}
					<li>{sessions.selectedSession.name.trim().split(' ')[0]}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-4">
		<label class="bg-accent text-accent-content flex items-center rounded-none border-1 px-2">
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

{#if sessions.data.length === 0}
	<div class="alert alert-warning">Please create a session first</div>
{/if}

{#if sessions.selectedSession?.id && classes.get_by_current_session()?.length > 0}
	<div class="flex justify-between">
		<div class="border-base-content/5 bg-base-100 overflow-x-auto rounded-none border w-1/2">
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
					{#each classes.get_by_current_session() as cls, i (i)}
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
        <div class="w-1/2">


        </div>
	</div>
{/if}

{#if sessions.selectedSession && classes.get_by_current_session()?.length === 0}
	<p>No classes yet. Click 'Create Class' to get started!</p>
{/if}

<CreateSection />
<CreateSubject />
<ListSubject />
<ClassEdit {selectedClass} />

<CreateClass />
