<script lang="ts">
	import { goto } from '$app/navigation';
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import ListSubject from '$lib/components/classes/ListSubject.svelte';
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { classes, classSubjects, sections, subjects } from '$lib/store/class.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { toast } from '$lib/store/toast.svelte';
	import type { Class, ClassSubject } from '$lib/types/class';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	const { data } = $props();

	interface SelectedSubject {
		id?: number;
		subject_id: number;
		isMandatory: boolean;
	}

	let selectedSubjects = $state<SelectedSubject[]>([]);

	let isEditing = $state<boolean>(false);
	let selectedClass: number | null = $state<number | null>(null);

	onMount(() => {
		selectedClass = Number(data.selectedClass);
		if (subjects.data.length === 0) {
			subjects.fetch();
		}
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

	let selectedClassData: Class | null = $state(null);
	$effect(() => {
		if (selectedClass) {
			selectedClassData = classes.get(sessions.selected as number, selectedClass) as Class;
		}
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

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm font-semibold">
			<ul>
				<li>Classes</li>
				{#if sessions.selectedSession}
					<li>{sessions.selectedSession.name.trim().split(' ')[0]}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex w-full justify-end gap-2">
		<div>
			<select
				class="select select-sm select-accent"
				bind:value={sessions.selected}
				on:change={(e) => sessions.select(Number((e.target as HTMLOptionElement).value))}
			>
				{#each sessions.data as session (session.id)}
					<option value={session.id}>{session.name}</option>
				{/each}
			</select>
		</div>

		<button
			class="btn btn-primary btn-sm"
			on:click={() =>
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal()}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Section
		</button>

		<button
			class="btn btn-secondary btn-sm"
			on:click={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Create Class
		</button>

		<button
			class="btn btn-info btn-sm"
			on:click={() =>
				(document.getElementById('list-subject-modal') as HTMLDialogElement).showModal()}
		>
			<Icon icon="duo-icons:book-2" font-size="20" />
			Subjects
		</button>
	</div>
</Navbar>

{#if sessions.data.length === 0}
	<div class="alert alert-warning mt-4">Please create a session first</div>
{:else if sessions.selectedSession}
	{#if classes.get_by_current_session()?.length > 0}
		<div class="mt-4 flex flex-col gap-2 xl:flex-row">
			<div class="w-full xl:w-1/2">
				<div class="bg-base-100 border-base-300 max-h-[85vh] w-full overflow-auto rounded border">
					<div class="overflow-x-auto">
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
										<td>40</td>
										<td>{sections.get_by_class(cls.id).length}</td>
										<td>{classSubjects.get(cls.id).length}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
			<div class="w-full xl:w-1/2">
				<div class="bg-base-100 border-base-300 text-accent w-full rounded border p-4">
					<h2 class="text-primary mb-3 text-xl font-bold">Class Details</h2>

					{#if selectedClass}
						<div class="space-y-3 text-sm">
							<div class="flex w-full">
								<div class="w-1/2">
									<div class="space-y-2">
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
											<p class="font-medium">
												{formatMoney(selectedClassData?.admission_fee as number)}
											</p>
										</div>

										<div>
											<p class="text-secondary">Monthly Fee</p>
											<p class="font-medium">
												{formatMoney(selectedClassData?.monthly_fee as number)}
											</p>
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
											on:click={() => {
												isEditing = true;
												(document.getElementById('create-class-modal') as HTMLDialogElement).show();
											}}
										>
											Edit
										</button>
										<button class="btn btn-error btn-sm" on:click={deleteClass}> Delete</button>
									</div>
								</div>

								<div class="space-y-2">
									<div>
										<p class="text-secondary">Total Students</p>
										<p class="font-medium">[DUMMY]</p>
									</div>
									<div>
										<p class="text-secondary">Total Sections</p>
										<p class="font-medium">[DUMMY]</p>
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

							<div class="grid grid-cols-2 gap-2 text-sm">
								<div>
									{#if subjects.data.length > 0}
										<h3 class="mb-3 text-lg font-bold">Subjects</h3>
										<ul class="bg-base-300 max-h-54 space-y-2 overflow-y-auto rounded p-2 text-sm">
											{#each subjects.data as subject, i (i)}
												<li class="bg-base-200 rounded p-2">
													<div class="flex items-center justify-between">
														<label class="flex cursor-pointer items-center gap-2">
															<input
																type="checkbox"
																class="checkbox checkbox-sm"
																checked={isSelected(subject.id)}
																on:change={() => toggleSubject(subject.id)}
															/>
															<span class="font-medium">{subject.name} • {subject.code}</span>
														</label>

														{#if isSelected(subject.id)}
															<label class="flex cursor-pointer items-center gap-1 text-xs">
																<input
																	type="checkbox"
																	class="checkbox checkbox-xs"
																	checked={isMandatory(subject.id)}
																	on:change={() => toggleMandatory(subject.id)}
																/>
																Mandatory
															</label>
														{/if}
													</div>
												</li>
											{/each}
										</ul>
									{:else}
										<p class="text-secondary alert alert-warning text-sm">
											You haven’t created any subjects yet.
										</p>
									{/if}

									<div class="mt-2 flex justify-end">
										{#if subjects.data.length > 0}
											<button
												class="btn btn-primary btn-sm"
												on:click={(e) => {
													e.preventDefault();
													submit();
												}}
											>
												Save
											</button>
										{/if}
									</div>
								</div>
								<div>
									<div class="">
										{#if sections.get_by_class(selectedClass).length > 0}
											<h3 class="mb-3 text-lg font-bold">Sections</h3>
											<ul
												class="bg-base-300 max-h-54 space-y-2 overflow-y-auto rounded p-2 text-sm"
											>
												{#each sections.get_by_class(selectedClass) as section, i (i)}
													<li class="bg-base-200 flex items-center justify-between rounded p-2">
														<span>{section.name}</span>
														<button
															class="btn btn-error btn-xs"
															on:click={() => deleteSection(section.id)}>Delete</button
														>
													</li>
												{/each}
											</ul>
										{:else}
											<p class="text-secondary alert alert-warning text-sm">
												You haven't created any sections yet.
											</p>
										{/if}
									</div>
								</div>
							</div>
						</div>
					{:else}
						<p class="text-secondary alert alert-warning text-sm">
							Select a class to view details.
						</p>
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<p class="alert alert-warning mt-4 text-sm">
			No classes yet. Click 'Create Class' to get started!
		</p>
	{/if}
{/if}

<CreateSection />
<CreateSubject />
<ListSubject />
<CreateClass bind:isEditing {selectedClass} />
