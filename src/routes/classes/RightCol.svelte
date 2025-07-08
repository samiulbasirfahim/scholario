<script lang="ts">
	import { goto } from '$app/navigation'; import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { students } from '$lib/store/student.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import type { Class, ClassSubject } from '$lib/types/class';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let { selectedClass, isEditing = $bindable() } = $props();

	let selectedClassData: Class = $derived(
		classes.get(sessions.selected as number, selectedClass) as Class
	);

	interface SelectedSubject {
		id?: number;
		subject_id: number;
		isMandatory: boolean;
	}

	let selectedSubjects = $state<SelectedSubject[]>([]);

	onMount(() => {
		subjects.fetch();
	});

	$effect(() => {
		if (selectedClass) {
			goto('?selectedClass=' + selectedClass, { replaceState: true });
			classSubjects.fetch(selectedClass);
			let selected: SelectedSubject[] = [];
			classSubjects.get(selectedClass).forEach((s) => {
				selected.push({
					id: s.id,
					subject_id: s.subject_id,
					isMandatory: s.is_mandatory
				});
			});
			selectedSubjects = selected;
		}
	});

	const toggleSubject = (subject_id: number) => {
		const exists = selectedSubjects.some((s) => s.subject_id === subject_id);
		if (exists) {
			selectedSubjects = selectedSubjects.filter((s) => s.subject_id !== subject_id);
		} else {
			selectedSubjects = [...selectedSubjects, { subject_id, isMandatory: false }];
		}

		submit();
	};

	const toggleMandatory = (id: number) => {
		selectedSubjects = selectedSubjects.map((s) =>
			s.subject_id === id ? { ...s, isMandatory: !s.isMandatory } : s
		);
		submit();
	};

	const isSelected = (id: number) => selectedSubjects.some((s) => s.subject_id === id);
	const isMandatory = (id: number) =>
		selectedSubjects.find((s) => s.subject_id === id)?.isMandatory ?? false;
	const submit = async () => {
		if (!selectedClass || selectedSubjects.length === 0) {
			toast.set({ message: 'Please select at least one subject', type: 'warning' });
			return;
		}

		const existing = classSubjects.get(selectedClass);
		const selected = selectedSubjects;

		const existingMap = new Map<number, ClassSubject>();
		existing.forEach((e) => existingMap.set(e.subject_id, e));

		const added: SelectedSubject[] = [];
		const changed: SelectedSubject[] = [];

		for (const sel of selected) {
			const prev = existingMap.get(sel.subject_id);
			if (!prev) {
				added.push(sel);
			} else if (prev.is_mandatory !== sel.isMandatory) {
				changed.push({ ...sel, id: prev.id });
			}
		}

		const removed = existing.filter((e) => !selected.some((s) => s.subject_id === e.subject_id));

		try {
			for (const { subject_id, isMandatory } of added) {
				const created = await invoke<ClassSubject>('create_class_subject', {
					class_id: selectedClass,
					subject_id,
					is_mandatory: isMandatory
				});

				classSubjects.add(selectedClass, created);
			}

			for (const { id, subject_id, isMandatory } of changed) {
				if (id != null) {
					const updated = await invoke<ClassSubject>('edit_class_subject', {
						id,
						class_id: selectedClass,
						subject_id,
						is_mandatory: isMandatory
					});

					const list = classSubjects.data[selectedClass];
					const index = list.findIndex((s) => s.id === id);
					if (index !== -1) {
						classSubjects.data[selectedClass][index] = updated;
					}
				}
			}

			for (const r of removed) {
				await invoke('delete_class_subject', { id: r.id });
				classSubjects.remove(selectedClass, r.id);
			}

			toast.set({ message: 'Subjects linked to class successfully', type: 'success' });
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to link subjects to class', type: 'error' });
		}
	};

	function formatMoney(amount: number): string {
		return (
			'৳' +
			(amount / 100).toLocaleString('en-BD', {
				style: 'decimal',
				minimumFractionDigits: 2
			})
		);
	}

	const deleteSection = async (id: number) => {
		try {
			await invoke('delete_section', { id });
			sections.remove(id);
			toast.set({ message: 'Section deleted', type: 'success' });
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to delete section', type: 'error' });
		}
	};

	const deleteClass = async () => {
		try {
			await invoke('delete_class', {
				id: selectedClass,
				session_id: sessions.selected as number
			});
			classes.remove(sessions.selected as number, selectedClass as number);
			toast.set({ message: 'Class deleted', type: 'success' });
		} catch (err) {
			console.log(err);
			toast.set({ message: 'Failed to delete class', type: 'error' });
		}
	};
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 text-accent w-full overflow-scroll rounded p-4">
		<h2
			class="border-accent mb-3 flex items-center justify-between border-b-1 pb-2 text-xl font-bold"
		>
			Class Details
		</h2>
		{#if selectedClass}
			<div class="space-y-3 text-sm">
				<div class="flex w-full">
					<div class="w-1/2">
						<div class="space-y-3">
							<div>
								<p class="text-secondary">Name</p>
								<p class="font-medium">{selectedClassData?.name}</p>
							</div>
							<div>
								<p class="text-secondary">Level</p>
								<p class="font-medium">{selectedClassData?.level}</p>
							</div>
							<div>
								<p class="text-secondary">Admission Fee</p>
								<p class="font-medium">{formatMoney(selectedClassData?.admission_fee as number)}</p>
							</div>
							<div>
								<p class="text-secondary">Monthly Fee</p>
								<p class="font-medium">{formatMoney(selectedClassData?.monthly_fee as number)}</p>
							</div>
							<div>
								<p class="text-secondary">Re-admission Fee</p>
								<p class="font-medium">
									{formatMoney(selectedClassData?.readmission_fee as number)}
								</p>
							</div>
						</div>
						<div class="flex flex-wrap gap-3 pt-2">
							<button
								class="btn btn-info btn-sm"
								onclick={() => {
									isEditing = true;
									(document.getElementById('create-class-modal') as HTMLDialogElement).show();
								}}
							>
								Edit
							</button> <button class="btn btn-error btn-sm" onclick={deleteClass}> Delete</button>
						</div>
					</div>

					<div class="space-y-3">
						<div>
							<p class="text-secondary">Total Students</p>
							<p class="font-medium">
								{#await students.get(sessions.selected as number, selectedClass) then students}
									{students.length}
								{/await}
							</p>
						</div>
						<div>
							<p class="text-secondary">Total Sections</p>
							<p class="font-medium">
								{sections.get_by_class(selectedClass).length}
							</p>
						</div>
						<div>
							<p class="text-secondary">Incoming</p>
							<p class="text-success font-medium">{formatMoney(10000)}</p>
						</div>
						<div>
							<p class="text-secondary">Outgoing</p>
							<p class="text-error font-medium">{formatMoney(10000)}</p>
						</div>
						<div class="col-span-2">
							<p class="text-secondary">Unpaid</p>
							<p class="text-info font-medium">{formatMoney(10000)}</p>
						</div>
					</div>
				</div>

				<hr class="border-base-300 my-3" />

				<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
					<div>
						<h3 class="mb-3 text-xl font-semibold text-gray-800">Subjects</h3>

						{#if subjects.data.length > 0}
							<ul
								class="bg-base-200 max-h-60 space-y-2 overflow-y-auto rounded border p-3 shadow-sm"
							>
								{#each subjects.data as subject, i (i)}
									<li class="bg-base-100 flex items-start justify-between rounded p-3 shadow">
										<label class="flex cursor-pointer items-start gap-3">
											<input
												type="checkbox"
												class="checkbox checkbox-sm mt-1"
												checked={isSelected(subject.id)}
												onchange={() => toggleSubject(subject.id)}
											/>
											<span class="text-sm font-medium">{subject.name} • {subject.code}</span>
										</label>

										{#if isSelected(subject.id)}
											<label class="flex items-center gap-2 text-xs text-gray-600">
												<input
													type="checkbox"
													class="checkbox checkbox-xs"
													checked={isMandatory(subject.id)}
													onchange={() => toggleMandatory(subject.id)}
												/>
												Mandatory
											</label>
										{/if}
									</li>
								{/each}
							</ul>
						{:else}
							<div class="alert alert-warning text-sm">You haven’t created any subjects yet.</div>
						{/if}
					</div>

					<!-- Sections -->
					<div>
						<h3 class="mb-3 text-xl font-semibold text-gray-800">Sections</h3>

						{#if sections.get_by_class(selectedClass).length > 0}
							<ul
								class="bg-base-200 max-h-60 space-y-2 overflow-y-auto rounded border p-3 shadow-sm"
							>
								{#each sections.get_by_class(selectedClass) as section, i (i)}
									<li class="bg-base-100 flex items-center justify-between rounded p-3 shadow">
										<span class="text-sm font-medium text-gray-700">{section.name}</span>
										<button class="btn btn-xs btn-error" onclick={() => deleteSection(section.id)}>
											Delete
										</button>
									</li>
								{/each}
							</ul>
						{:else}
							<div class="alert alert-warning text-sm">You haven't created any sections yet.</div>
						{/if}
					</div>
				</div>
			</div>
		{:else}
			<p class="text-secondary alert alert-warning text-sm">Select a class to view details.</p>
		{/if}
	</div>
</div>
