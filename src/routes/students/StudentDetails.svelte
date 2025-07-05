<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';
	import { guardians, studentRelationships } from '$lib/store/guardian.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';

	type Guardian = {
		id: number;
		name: string;
		relation: string;
		phone: string;
		address: string;
		photo: string;
	};

	let { selectedStudentData, selectedStudent, isEditing = $bindable() } = $props();

	let guardians_s = $state<Guardian[]>([]);

	$effect(() => {
		console.log(studentRelationships.reactiveCounter);
		(async () => {
			if (selectedStudent) {
				await studentRelationships.fetch(selectedStudent);
				const srls = studentRelationships.get(selectedStudent);

				const guardiansList: Guardian[] = srls.map((srl) => ({
					...(guardians.get(srl.related_id) as Guardian),
					relation: srl.relationship as string
				}));

				guardians_s = [
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList,
					...guardiansList
				];
			}
		})();
	});

	const deleteStudent = async () => {
		try {
			if (selectedStudent) {
				await invoke('delete_student', {
					id: selectedStudent,
					session_id: sessions.selected as number
				});
				students.remove(selectedStudent);
				selectedStudent = null;
				toast.set({ message: 'Student deleted', type: 'success' });
			}
		} catch (err) {
			console.log(err);
			toast.set({ message: 'Failed to delete student', type: 'error' });
		}
	};
</script>

<div class="flex w-full flex-1 flex-col gap-0 overflow-hidden xl:w-1/2">
	<div class="bg-base-100 flex flex-col rounded p-4 overflow-hidden">
		<div
			class="border-accent mb-3 flex items-center justify-between border-b-1 pb-2 text-xl font-bold"
		>
			<h1 class="text-primary">Student Details</h1>
			<div class="join">
				<button
					class="btn btn-warning tooltip tooltip-bottom btn-sm join-item text-xl"
					data-tip="Edit"
					onclick={() => {
						isEditing = true;
						(document.getElementById('create-student-modal') as HTMLDialogElement).show();
					}}
				>
					<Icon icon="uil:edit" font-size="22" />
				</button>

				<button
					class="btn btn-error tooltip tooltip-bottom btn-sm join-item text-xl"
					data-tip="Delete"
					onclick={deleteStudent}
				>
					<Icon icon="mdi:delete" />
				</button>
			</div>
		</div>

		{#if selectedStudent && selectedStudentData}
			<div class="text-sm">
				<div class="text-accent grid grid-cols-2 gap-2">
					<div class="space-y-2">
						<div>
							<p class="text-secondary">Name</p>
							<p class="font-medium">{selectedStudentData.name}</p>
						</div>

						<div>
							<p class="text-secondary">Gender</p>
							<p class="font-medium">{selectedStudentData.gender}</p>
						</div>

						<div>
							<p class="text-secondary">Date of Birth</p>
							<p class="font-medium">{selectedStudentData.dob}</p>
						</div>

						<div>
							<p class="text-secondary">Phone</p>
							<p class="font-medium">
								{selectedStudentData.phone || 'N/A'}
							</p>
						</div>

						<div>
							<p class="text-secondary">Admission Date</p>
							<p class="font-medium">{selectedStudentData.admission_date}</p>
						</div>

						<div>
							<p class="text-secondary">Class</p>
							<p class="font-medium">
								{classes.get(sessions.selected as number, selectedStudentData.class_id as number)
									?.name}
							</p>
						</div>

						{#if selectedStudentData.section_id !== null}
							<div>
								<p class="text-secondary">Section</p>
								<p class="font-medium">
									{sections.get(selectedStudentData.section_id)?.name}
								</p>
							</div>
						{/if}

						<div>
							<p class="text-secondary">Roll</p>
							<p class="font-medium">{selectedStudentData.roll}</p>
						</div>

						<div>
							<p class="text-secondary">Resident</p>
							<p class="font-medium">
								{selectedStudentData.is_resident ? 'Yes' : 'No'}
							</p>
						</div>

						<div>
							<p class="text-secondary">Religion</p>
							<p class="font-medium">{selectedStudentData.religion}</p>
						</div>
					</div>
					<div class="space-y-2">
						<img
							src={selectedStudentData.photo}
							alt="Photo of {selectedStudentData.name}"
							class="h-28 w-28 flex-shrink-0 rounded object-cover"
						/>

						{#if selectedStudentData.health_notes}
							<div>
								<p class="text-secondary text-xs">Health Notes</p>
								<div
									class="bg-base-200 overflow-y-auto rounded p-2"
									style="max-height: 80px; transform: translateZ(0);"
								>
									<p class="text-sm leading-snug font-medium break-words antialiased">
										{selectedStudentData.health_notes}
									</p>
								</div>
							</div>
						{/if}

						{#if selectedStudentData.general_notes}
							<div>
								<p class="text-secondary text-xs">General Notes</p>
								<div
									class="bg-base-200 overflow-y-auto rounded p-2"
									style="max-height: 80px; transform: translateZ(0);"
								>
									<p class="text-sm leading-snug font-medium break-words antialiased">
										{selectedStudentData.general_notes}
									</p>
								</div>
							</div>
						{/if}

						<div>
							<p class="text-secondary text-xs">Address</p>
							<div
								class="bg-base-200 overflow-y-auto rounded p-2"
								style="max-height: 80px; transform: translateZ(0);"
							>
								<p class="text-sm leading-snug font-medium break-words antialiased">
									{selectedStudentData.address}
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<p class="text-secondary alert alert-warning text-sm">Select a student to view details.</p>
		{/if}
		{#if guardians_s.length > 0}
			<div class="border-accent flex flex-1 flex-col overflow-hidden mt-4">
				<h2 class="text-primary border-accent mb-3 border-b-1 pb-2 text-xl font-bold">Guardians</h2>
				<ul class="grid flex-1 grid-cols-2 gap-4 overflow-auto">
					{#each guardians_s as g, i ((g.id, i))}
						<li class="bg-base-300 flex items-center gap-4 rounded p-2 shadow-sm">
							<div class="size-12 flex-shrink-0 overflow-hidden rounded-full">
								<img
									src={g.photo}
									alt={`Photo of ${g.name}`}
									class="h-full w-full object-cover object-center"
								/>
							</div>
							<div class="min-w-0 flex-1">
								<p class="truncate font-medium">{g.name}</p>
								<p class="truncate text-sm text-gray-500">
									{g.relation ?? '—'} • {g.phone}
								</p>
								<p class="truncate text-sm text-gray-500">{g.address ?? 'No address'}</p>
							</div>
						</li>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
