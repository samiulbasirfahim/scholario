<script lang="ts">
	import { classes, classSubjects, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';

	let { selectedClass = $bindable() } = $props();
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 w-full overflow-scroll rounded">
		<table class="table">
			<thead class="bg-base-200">
				<tr>
					<th>#</th>
					<th>Name</th>
					<th>Students</th>
					<th>Sections</th>
					<th>Subjects</th>
				</tr>
			</thead>
			<tbody>
				{#each classes.get_by_current_session() as cls, i (cls.id)}
					<tr
						class="{cls.id === selectedClass
							? 'bg-primary text-primary-content'
							: ''} cursor-pointer"
						on:click={() => {
							selectedClass = cls.id;
						}}
					>
						<th>{i + 1}</th>
						<td>{cls.name}</td>
						<td>
							{#await students.get(sessions.selected, cls.id) then students}
								{students.length}
							{/await}
						</td>
						<td>{sections.get_by_class(cls.id).length}</td>
						<td>{classSubjects.get(cls.id).length}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
