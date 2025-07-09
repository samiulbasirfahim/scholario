<script lang="ts">
	import { subjects } from '$lib/store/class.svelte';
	import Icon from '@iconify/svelte';
	import EditSubject from './EditSubject.svelte';

	let selectedSubject: number | undefined = undefined;

	const openEditModal = (id: number) => {
		selectedSubject = id;
		const modal = document.getElementById('edit-subject-modal') as HTMLDialogElement;
		modal?.showModal();
	};
</script>

<dialog id="list-subject-modal" class="modal">
	<div class="modal-box w-full max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>

		<h3 class="mb-4 text-lg font-bold">Subjects</h3>

		{#if subjects.data.length > 0}
			<div class="max-h-80 overflow-x-auto">
				<table class="table-pin-rows table-sm table">
					<thead>
						<tr>
							<th class="w-6">#</th>
							<th>Name</th>
							<th>Code</th>
							<th class="w-12"></th>
						</tr>
					</thead>
					<tbody>
						{#each subjects.data as subject, i (subject.id)}
							<tr>
								<th>{i + 1}</th>
								<td>{subject.name}</td>
								<td>{subject.code}</td>
								<td>
									<button
										class="btn btn-ghost hover:btn-primary btn-square"
										aria-label="Edit Subject"
										onclick={() => openEditModal(subject.id)}
										type="button"
									>
										<Icon icon="lucide:edit" font-size="20" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else}
			<div class="alert alert-warning">No subjects yet. Click 'Create' to get started!</div>
		{/if}

		<div class="mt-4 flex justify-end">
			<button
				class="btn btn-primary flex items-center gap-2"
				type="button"
				onclick={() => {
					const modal = document.getElementById('create-subject-modal') as HTMLDialogElement;
					modal?.showModal();
				}}
			>
				<Icon icon="gridicons:create" font-size="20" />
				Create
			</button>
		</div>
	</div>

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button aria-label="Close modal" class="h-full w-full" />
	</form>
</dialog>

<EditSubject {selectedSubject} />
