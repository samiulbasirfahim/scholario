<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { Confirm } from '$lib/utility/Confirm';

	let { deleteStudent, students_d, selectedStudent = $bindable() } = $props();

	let selected_students = $state<number[]>([]);
	let is_selecting = $state<boolean>(false);

	function toggleSelection() {
		is_selecting = !is_selecting;
		if (!is_selecting) selected_students = [];
	}

	function change_place() {
		console.log('Promoting:', selected_students);
	}

	async function deleteStudents() {
		let answer: boolean = await Confirm();
		if (!answer) return;

		for (const id in selected_students) {
			deleteStudent(Number(id));
		}
	}
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 w-full overflow-auto rounded">
		<div class="h-full overflow-auto">
			<table class="table-pin-rows table">
				<thead>
					<tr class="bg-base-200">
						{#if selected_students.length > 0 || is_selecting}
							<th class="w-3"></th>
						{/if}
						<th class="w-4">Roll</th>
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
							{#if selected_students.length > 0 || is_selecting}
								<td>
									<input
										type="checkbox"
										class="checkbox checkbox-sm"
										value={student.id}
										bind:group={selected_students}
										onclick={(e) => e.stopPropagation()}
									/>
								</td>
							{/if}
							<td>{student.roll}</td>
							<td>{student.name}</td>
							<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
							<td>
								{#if student.section_id >= 0}
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

<div class="fixed right-4 bottom-4 z-50">
	{#if is_selecting}
		<div class="bg-base-100 w-64 rounded-lg border p-3 shadow-xl">
			<h3 class="mb-2 text-base font-semibold">
				Selected: <span class="text-primary font-bold">{selected_students.length}</span>
			</h3>

			<div class="mb-2 grid grid-cols-1 gap-2">
				<button class="btn btn-sm btn-accent" onclick={change_place}>Change Placement</button>
				<button class="btn btn-sm btn-error" onclick={deleteStudents}>Delete</button>
			</div>

			<div class="flex justify-between gap-2">
				<button class="btn btn-sm btn-info flex-1">Select All</button>
				<button class="btn btn-sm btn-warning flex-1">Clear</button>
			</div>

			<button
				class="mt-3 w-full btn-sm btn font-medium transition"
				onclick={toggleSelection}
			>
				Cancel
			</button>
		</div>
	{:else}
		<button class="btn btn-primary btn-sm rounded px-4 py-2 shadow-md" onclick={toggleSelection}>
			Bulk Edit
		</button>
	{/if}
</div>
