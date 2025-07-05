<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';

	let {
		filter,
		hasDateExceededEndDate,
		selectedStudent = $bindable(),
		attendance = $bindable(),
		students_d = $bindable()
	} = $props();

	type AttendanceStatus = 'present' | 'absent' | 'late';

	function takeAttendance(id: number, status: AttendanceStatus) {
		attendance = { ...attendance, [id]: status };
	}
</script>

<div class="w-full flex-1 xl:w-1/2 overflow-hidden flex flex-col">
	<div class="bg-base-100 rounded overflow-scroll w-full">
		<table class="table-pin-rows table">
			<thead>
				<tr class="bg-base-200">
					<th class="w-4">Roll</th>
					<th>Name</th>
					{#if filter.class === ''}
						<th>Class</th>
					{/if}
					{#if filter.section === ''}
						<th>Section</th>
					{/if}

					{#if !hasDateExceededEndDate()}
						<th>Status</th>
					{/if}
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
						{#if filter.class === ''}
							<td>{classes.get(sessions.selected as number, student.class_id)?.name}</td>
						{/if}
						{#if filter.section === ''}
							<td>
								{#if student.section_id}
									{sections.get(student.section_id)?.name}
								{:else}
									-
								{/if}
							</td>
						{/if}

						{#if !hasDateExceededEndDate()}
							<td>
								<select
									class="select bg-secondary text-secondary-content select-sm"
									onchange={(e) => {
										takeAttendance(
											student.id,
											(e.target as HTMLSelectElement).value as AttendanceStatus
										);
									}}
								>
									<option disabled selected>Pick status</option>
									<option value="present">Present</option>
									<option value="late">Late</option>
									<option value="absent">Absent</option>
								</select>
							</td>
						{/if}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
