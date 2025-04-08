<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
	import { subjects } from '$lib/store/class.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import Toast from '../global/Toast.svelte';

	let { selectedClass } = $props();

	interface SelectedSubject {
		id: number;
		isMandatory: boolean;
	}

	let selectedSubjects = $state<SelectedSubject[]>([]);

	onMount(() => {
		if (subjects.subjects.length === 0) {
			subjects.fetch();
		}
	});

	const toggleSubject = (id: number) => {
		const exists = selectedSubjects.some((s) => s.id === id);
		if (exists) {
			selectedSubjects = selectedSubjects.filter((s) => s.id !== id);
		} else {
			selectedSubjects = [...selectedSubjects, { id, isMandatory: false }];
		}
	};

	const toggleMandatory = (id: number) => {
		selectedSubjects = selectedSubjects.map((s) =>
			s.id === id ? { ...s, isMandatory: !s.isMandatory } : s
		);
	};

	const isSelected = (id: number) => selectedSubjects.some((s) => s.id === id);
	const isMandatory = (id: number) =>
		selectedSubjects.find((s) => s.id === id)?.isMandatory ?? false;

	const submit = async () => {
		if (!selectedClass || selectedSubjects.length === 0) {
			toast.set({ message: 'Please select at least one subject', type: 'warning' });
			return;
		}

		try {
			for (const { id, isMandatory } of selectedSubjects) {
				await invoke('create_class_subject', {
					class_id: selectedClass,
					subject_id: id,
					is_mandatory: isMandatory
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
			{#if subjects.subjects.length > 0}
				{#each subjects.subjects as subject, i (i)}
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

			{#if subjects.subjects.length > 0}
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
