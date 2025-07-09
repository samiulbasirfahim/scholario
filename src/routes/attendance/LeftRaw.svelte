<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';

	let {
		filter,
		hasDateExceededEndDate,
		selectedStudent = $bindable(),
		attendance = $bindable(),
		students_d = $bindable(),
		staff_d = $bindable(),
		for_whom = $bindable()
	} = $props();

	type AttendanceStatus = 'present' | 'absent' | 'late';

	function takeAttendance(id: number, status: AttendanceStatus) {
		attendance = { ...attendance, [id]: status };
	}
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 w-full overflow-scroll rounded">
		<table class="table-pin-rows table">
			<thead>
				<tr class="bg-base-200">
					{#if for_whom === 'STUDENT'}
						<th class="w-4">Roll</th>
					{/if}
					<th>Name</th>

					{#if for_whom === 'STUDENT'}
						{#if filter.class === ''}
							<th>Class</th>
						{/if}
						{#if filter.section === ''}
							<th>Section</th>
						{/if}
					{:else}
						<th>Role</th>
					{/if}

					{#if !hasDateExceededEndDate()}
						<th>Status</th>
					{/if}
				</tr>
			</thead>
			<tbody>
				{#if for_whom === 'STUDENT'}
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
								<td>{student.section_id ? sections.get(student.section_id)?.name : '-'}</td>
							{/if}

							{#if !hasDateExceededEndDate()}
								<td class="max-w-24">
									<select
										class="select select-sm text-base-content"
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
				{:else}
					{#each staff_d as staffMember, i ((staffMember.id, i))}
						<tr
							class="{staffMember.id === selectedStudent
								? 'bg-primary text-primary-content'
								: ''} cursor-pointer"
							onclick={() => {
								selectedStudent = staffMember.id;
							}}
						>
							<td>{staffMember.name}</td>
							<td>{staffMember.role}</td>

							{#if !hasDateExceededEndDate()}
								<td class="max-w-24">
									<select
										class="select select-sm text-base-content"
										onchange={(e) => {
											takeAttendance(
												staffMember.id,
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
				{/if}
			</tbody>
		</table>
	</div>
</div>
