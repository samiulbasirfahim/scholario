<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import Toast from '../global/Toast.svelte';
	import type { Class, ClassSubject } from '$lib/types/class';

	let { selectedClass } = $props();

	interface SelectedSubject {
		id?: number;
		subject_id: number;
		isMandatory: boolean;
	}

	let selectedSubjects = $state<SelectedSubject[]>([]);

	onMount(() => {
		if (subjects.data.length === 0) {
			subjects.fetch();
		}
	});

	$effect(() => {
		if (selectedClass != null) classSubjects.fetch(selectedClass);
		let selected: SelectedSubject[] = [];
		classSubjects.get(selectedClass).forEach((s) => {
			selected.push({
				id: s.id,
				subject_id: s.subject_id,
				isMandatory: s.is_mandatory
			});
		});
		selectedSubjects = selected;
	});

	const toggleSubject = (subject_id: number) => {
		const exists = selectedSubjects.some((s) => s.subject_id === subject_id);
		if (exists) {
			selectedSubjects = selectedSubjects.filter((s) => s.subject_id !== subject_id);
		} else {
			selectedSubjects = [...selectedSubjects, { subject_id, isMandatory: false }];
		}
	};

	const toggleMandatory = (id: number) => {
		selectedSubjects = selectedSubjects.map((s) =>
			s.subject_id === id ? { ...s, isMandatory: !s.isMandatory } : s
		);
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

				classSubjects.add(selectedClass, created); // ✅ Update store
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
				classSubjects.remove(selectedClass, r.id); // ✅ Update store
			}

			toast.set({ message: 'Subjects linked to class successfully', type: 'success' });

			selectedSubjects = [];
			// (document.getElementById('link-subjects-modal') as HTMLDialogElement).close();
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to link subjects to class', type: 'error' });
		}
	};

	let className = $state('');
	let level = $state('');
	let admissionFee = $state('');
	let monthlyFee = $state('');
	let readmissionFee = $state('');

	$effect(() => {
		if (selectedClass != null) {
			const cls = classes.get(selectedClass);
			if (cls) {
				className = cls.name;
				level = cls.level;
				admissionFee = (cls.admission_fee / 100).toString();
				monthlyFee = (cls.monthly_fee / 100).toString();
				readmissionFee = (cls.readmission_fee / 100).toString();
			}
		}
	});

	const updateClassDetails = async () => {
		if (!className.trim()) {
			toast.set({ message: 'Class name is required', type: 'warning' });
			return;
		}

		try {
			const updated = await invoke('edit_class', {
				id: selectedClass,
				name: className,
				level,
				admission_fee: parseInt(admissionFee) * 100,
				monthly_fee: parseInt(monthlyFee) * 100,
				readmission_fee: parseInt(readmissionFee) * 100
			});

			classes.update(selectedClass, updated as Class);
			toast.set({ message: 'Class updated successfully', type: 'success' });
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to update class', type: 'error' });
		}
	};

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
			await invoke('delete_class', { id: selectedClass });
			classes.remove(selectedClass);
			toast.set({ message: 'Class deleted', type: 'success' });
			(document.getElementById('class-edit-modal') as HTMLDialogElement).close();
		} catch (err) {
			console.log(err);
			toast.set({ message: 'Failed to delete class', type: 'error' });
		}
	};
</script>

<dialog id="class-edit-modal" class="modal">
	<div class="modal-box w-11/12 max-w-4xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<div class="grid grid-cols-2 gap-6">
			<div>
				<!-- Edit Class Details -->
				<div class="space-y-4">
					<h3 class="mb-2 text-lg font-bold">Edit Class Details</h3>

					<div class="grid grid-cols-2 gap-4">
						<div>
							<label for="name" class="block text-sm font-medium">Name</label>
							<input
								type="text"
								id="name"
								class="input input-bordered w-full"
								required
								bind:value={className}
							/>
						</div>

						<div>
							<label for="level" class="block text-sm font-medium">Level</label>
							<input
								type="text"
								id="level"
								class="input input-bordered w-full"
								required
								bind:value={level}
							/>
						</div>
						<div>
							<label for="admissionFee" class="block text-sm font-medium">Admission Fee</label>
							<input
								type="number"
								id="admissionFee"
								class="input input-bordered w-full"
								required
								bind:value={admissionFee}
							/>
						</div>

						<div>
							<label for="monthlyFee" class="block text-sm font-medium">Monthly Fee</label>
							<input
								type="number"
								id="monthlyFee"
								class="input input-bordered w-full"
								required
								bind:value={monthlyFee}
							/>
						</div>
						<div>
							<label for="readmissionFee" class="block text-sm font-medium">Readmission Fee</label>
							<input
								type="number"
								id="readmissionFee"
								class="input input-bordered w-full"
								required
								bind:value={readmissionFee}
							/>
						</div>
					</div>

					<div class="join flex justify-end">
						<button class="btn btn-error join-item" onclick={deleteClass}>Delete</button>
						<button class="btn btn-primary join-item" onclick={updateClassDetails}>Update</button>
					</div>
				</div>

				<!-- Delete Sections -->
				<div class="mt-8 space-y-4">
					<h3 class="mb-2 text-lg font-bold">Delete Sections</h3>

					{#if sections.get_by_class(selectedClass).length > 0}
						<ul class="space-y-2">
							{#each sections.get_by_class(selectedClass) as section, i (i)}
								<li class="flex items-center justify-between rounded border p-2">
									<span>{section.name}</span>
									<button class="btn btn-error btn-sm" onclick={() => deleteSection(section.id)}
										>Delete</button
									>
								</li>
							{/each}
						</ul>
					{:else}
						<p class="text-sm text-gray-500">No sections available.</p>
					{/if}
				</div>
			</div>
			<div>
				<h3 class="mb-4 text-lg font-bold">Assign Subjects to Class</h3>

				<div class="max-h-60 space-y-2 overflow-y-auto pr-2">
					{#if subjects.data.length > 0}
						{#each subjects.data as subject, i (i)}
							<div class="flex items-center justify-between gap-4">
								<label class="fieldset-label">
									<input
										type="checkbox"
										class="checkbox"
										checked={isSelected(subject.id)}
										onchange={() => toggleSubject(subject.id)}
									/>
									{subject.name} • {subject.code}
								</label>

								{#if isSelected(subject.id)}
									<label class="fieldset-label text-xs">
										<input
											type="checkbox"
											class="checkbox checkbox-xs"
											checked={isMandatory(subject.id)}
											onchange={() => toggleMandatory(subject.id)}
										/>
										Mandatory
									</label>
								{/if}
							</div>
						{/each}
					{:else}
						<p>You havent created any subjects yet</p>
					{/if}
				</div>

				<div class="mt-6 flex justify-end gap-2">
					<button
						class="btn btn-secondary"
						onclick={() => {
							(document.getElementById('create-subject-modal') as HTMLDialogElement).showModal();
						}}
					>
						<Icon icon="material-symbols:book" />
						Create Subject
					</button>

					{#if subjects.data.length > 0}
						<button
							class="btn btn-primary"
							onclick={(e) => {
								e.preventDefault();
								submit();
							}}
						>
							Save
						</button>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<Toast />
</dialog>
