<script lang="ts">
	let { guardians = $bindable() } = $props();

	function closeGuardiansModal() {
		const studentModal = document.getElementById('create-student-modal') as HTMLDialogElement;
		const guardianModal = document.getElementById('manage-guardians-modal') as HTMLDialogElement;

		guardianModal.close();
		studentModal.showModal();
	}
</script>

<dialog id="manage-guardians-modal" class="modal">
	<div class="modal-box w-11/12 max-w-4xl">
		<form method="dialog">
			<button
				class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2"
				onclick={(e) => {
					e.preventDefault();
					closeGuardiansModal();
				}}>✕</button
			>
		</form>
		<h3 class="mb-4 text-lg font-bold">Manage Guardians</h3>

		<div class="grid grid-cols-3 gap-6">
			<!-- Guardian List (2/3rd) -->
			<div class="col-span-2 max-h-[400px] space-y-2 overflow-y-auto">
				<!-- Replace with real guardian list -->
				<div class="bg-base-200 rounded p-2">Guardian 1</div>
				<div class="bg-base-200 rounded p-2">Guardian 2</div>
				<!-- ... -->
			</div>

			<!-- Create Guardian Form (1/3rd) -->
			<div>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						closeGuardiansModal();
					}}
				>
					<label class="mb-2 block text-sm font-medium">Name</label>
					<input class="input input-bordered mb-2 w-full" placeholder="Guardian Name" />
					<!-- Add more fields here -->
					<button class="btn btn-primary w-full">Create</button>
				</form>
			</div>
		</div>
	</div>
</dialog>
