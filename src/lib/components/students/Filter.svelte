<script lang="ts">
	import { page } from '$app/state';
	import { classes, sections } from '$lib/store/class.svelte';

	let { filter = $bindable({}) } = $props();

	function applyFilter() {
		console.log($state.snapshot(filter));
		(document.getElementById('filter-modal') as HTMLDialogElement)?.close();
	}

	$effect(() => {
		console.log();
	});
</script>

<dialog id="filter-modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<h3 class="mb-4 text-lg font-bold">FILTER</h3>
		<form
			class="space-y-2"
			onsubmit={(e) => {
				e.preventDefault();
				applyFilter();
			}}
		>
			<div>
				<label class="label" for="class">
					<span class="label-text text-sm font-medium">By Class</span>
				</label>
				<select bind:value={filter.class} class="select select-bordered w-full">
					<option value="">All Classes</option>
					{#each classes.get_by_current_session() as cls, i (i)}
						<option value={cls.id}>{cls.name}</option>
					{/each}
				</select>
			</div>

			{#if sections.get_by_class(Number(filter.class)).length > 0}
				<div>
					<label class="label" for="section">
						<span class="label-text text-sm font-medium">By Section</span>
					</label>
					<select bind:value={filter.section} class="select select-bordered w-full">
						<option value="">All Sections</option>

						{#each sections.get_by_class(Number(filter.class)) as section, i (i)}
							<option value={section.id}>{section.name}</option>
						{/each}
					</select>
				</div>
			{/if}

			{#if page.url.pathname !== '/attendance'}
				<div>
					<label class="label" for="sort">
						<span class="label-text text-sm font-medium">Fee Status</span>
					</label>
					<select bind:value={filter.fee} class="select select-bordered w-full">
						<option value="">None</option>
						<option value="unpaid-desc">Unpaid: High → Low</option>
						<option value="unpaid-asc">Unpaid: Low → High</option>
					</select>
				</div>

				<div>
					<label class="label" for="rollsort">
						<span class="label-text text-sm font-medium">Sort by Roll</span>
					</label>
					<select bind:value={filter.roll} class="select select-bordered w-full">
						<option value="">None</option>
						<option value="asc">Roll: Low → High</option>
						<option value="desc">Roll: High → Low</option>
					</select>
				</div>
			{/if}
		</form>
	</div>

	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button>close</button>
	</form>

</dialog>
