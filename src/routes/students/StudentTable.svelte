<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';

	let { students_d, selectedStudent = $bindable() } = $props();
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 w-full overflow-scroll rounded">
		<div class="h-full overflow-auto">
			<table class="table-pin-rows table">
				<thead>
					<tr class="bg-base-200">
						<th>Roll</th>
						<th>Name</th>
						<th>Class</th>
						<th>Section</th>
						<!-- <th>Roll</th> -->
					</tr>
				</thead>
				<tbody>
					{#each students_d as student, i ((student.id, i))}
						<tr
							class="{student.id === selectedStudent
								? 'bg-primary text-primary-content'
								: ''} cursor-pointer"
							onclick={() => {
								selectedStudent = student.id;
							}}
						>
							<td>{student.roll}</td>
							<td>{student.name}</td>
							<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
							<td>
								{#if student.section_id}
									{sections.get(student.section_id)?.name}
								{:else}
									-
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
