<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="navbar bg-base-300 rounded px-4">
	<div class="flex-1">
		<p>Classes & Subjects</p>
	</div>
	<div class="flex gap-2">
		<button
			class="btn btn-accent"
			onclick={() => {
				(document.getElementById('create-subject-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" />
			Create Subject
		</button>

		<button
			class="btn btn-secondary"
			onclick={() => {
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" />
			Create Section
		</button>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Create Class</button
		>
	</div>
</div>

<dialog id="create-class-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">Hello!</h3>
		<p class="py-4">Press ESC key or click on ✕ button to close</p>
	</div>
</dialog>

<dialog id="create-section-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">Hello!</h3>
		<p class="py-4">Press ESC key or click on ✕ button to close</p>
	</div>
</dialog>

<dialog id="create-subject-modal" class="modal">
	<div class="modal-box w-full max-w-xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">Create Subject</h3>

		<form
			onsubmit={(e) => {
				e.preventDefault();
			}}
			class="space-y-4"
		>
			<div class="form-control">
				<label class="label">
					<span class="label-text">Name</span>
				</label>
				<input type="text" class="input input-bordered w-full" required />
			</div>

			<div class="form-control">
				<label class="label">
					<span class="label-text">Code</span>
				</label>
				<input type="number" class="input input-bordered w-full" required />
			</div>

			<div class="form-control">
				<label class="label">
					<span class="label-text">Description</span>
				</label>
				<textarea class="textarea textarea-bordered w-full" rows="2"></textarea>
			</div>

			<div class="flex justify-end">
				<button type="submit" class="btn btn-primary">Create</button>
			</div>
		</form>
	</div>
</dialog>
