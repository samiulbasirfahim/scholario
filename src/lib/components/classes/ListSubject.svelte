<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { subjects } from '$lib/store/class.svelte';
	// import type { Subject } from '$lib/types/class';
	import Icon from '@iconify/svelte';
	import EditSubject from './EditSubject.svelte';
	let selectedSubject = $state<number | undefined>();
</script>

<dialog id="list-subject-modal" class="modal">
	<div class="modal-box w-full max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="mb-4 text-lg font-bold">Subjects</h3>
		{#if subjects.data.length > 0}
			<div class="overflow-x-auto">
				<table class="table">
					<thead>
						<tr>
							<th></th>
							<th>Name</th>
							<th>Code</th>
							<th></th>
						</tr>
					</thead>
					<tbody>
						<!-- row 1 -->
						{#each subjects.data as subject, i (i)}
							<tr>
								<th class="w-4">{i + 1}</th>
								<td>{subject.name}</td>
								<td>{subject.code}</td>
								<td>
									<button
										class="btn btn-ghost hover:btn-primary btn-square"
										onclick={() => {
											selectedSubject = subject.id;
											(
												document.getElementById('edit-subject-modal') as HTMLDialogElement
											).showModal();
										}}
									>
										<Icon icon="lucide:edit" font-size="24" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else}
			<p>No Subject yet, create Subject first</p>
		{/if}

		<button
            class="btn btn-primary mt-2"
			onclick={() => {
				(document.getElementById('create-subject-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Create
		</button>
	</div>
</dialog>

<EditSubject {selectedSubject} />
