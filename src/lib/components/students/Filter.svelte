<script lang="ts">
	import { classes, sections } from '$lib/store/class.svelte';

	let selectedClass = $state('');
	let selectedSection = $state('');
	let feeSort = $state('');

	let rollSort = $state('');

	function applyFilter() {
		console.log({ selectedClass, selectedSection, feeSort, rollSort });
		(document.getElementById('filter-modal') as HTMLDialogElement)?.close();
	}
</script>

<dialog id="filter-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="text-lg font-bold">FILTER</h3>
		<form
			class="space-y-4"
			onsubmit={(e) => {
				e.preventDefault();
				applyFilter();
			}}
		>
			<div>
				<label class="label" for="class">
					<span class="label-text font-medium">By Class</span>
				</label>
				<select bind:value={selectedClass} class="select select-bordered w-full">
					<option value="">All Classes</option>
					{#each classes.classes as cls, i (i)}
						<option value={cls.id}>{cls.name}</option>
					{/each}
				</select>
			</div>

			<div>
				<label class="label" for="section">
					<span class="label-text font-medium">By Section</span>
				</label>
				<select bind:value={selectedSection} class="select select-bordered w-full">
					<option value="">All Sections</option>

					{#each sections.sections.filter((section) => section.class_id === Number(selectedClass)) as section, i (i)}
						<option value={section.id}>{section.name}</option>
					{/each}
				</select>
			</div>

			<div>
				<label class="label" for="sort">
					<span class="label-text font-medium">Fee Status</span>
				</label>
				<select bind:value={feeSort} class="select select-bordered w-full">
					<option value="">None</option>
					<option value="unpaid-desc">Unpaid: High → Low</option>
					<option value="unpaid-asc">Unpaid: Low → High</option>
				</select>
			</div>

			<div>
				<label class="label" for="rollsort">
					<span class="label-text font-medium">Sort by Roll</span>
				</label>
				<select bind:value={rollSort} class="select select-bordered w-full">
					<option value="">None</option>
					<option value="asc">Roll: Low → High</option>
					<option value="desc">Roll: High → Low</option>
				</select>
			</div>
		</form>
	</div>
</dialog>
