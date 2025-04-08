<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { classSubjects, subjects } from '$lib/store/class.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import Toast from '../global/Toast.svelte';

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
		const exists = selectedSubjects.some((s) => s.id === subject_id);
		if (exists) {
			selectedSubjects = selectedSubjects.filter((s) => s.id !== subject_id);
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

		try {
			for (const { subject_id: id, isMandatory } of selectedSubjects) {
				await invoke('create_class_subject', {
					class_id: selectedClass,
					subject_id: id,
					is_mandatory: isMandatory
				});
			}

			for (const { id, subject_id, isMandatory } of selectedSubjects) {
				classSubjects.add(selectedClass, {
					id,
					is_mandatory: isMandatory,
					class_id: selectedClass,
					subject_id: subject_id
				});
			}

			toast.set({ message: 'Subjects linked to class successfully', type: 'success' });
			selectedSubjects = [];
			(document.getElementById('link-subjects-modal') as HTMLDialogElement).close();
		} catch (err) {
			console.error(err);
			toast.set({ message: 'Failed to link subjects to class', type: 'error' });
		}
	};
</script>

<dialog id="link-subjects-modal" class="modal">
	<div class="modal-box max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

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
					Submit
				</button>
			{/if}
		</div>
	</div>

	<Toast />
</dialog>
