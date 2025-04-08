<script lang="ts">
	import CreateStudent from '$lib/components/students/CreateStudent.svelte';
	import Filter from '$lib/components/students/Filter.svelte';
	import Icon from '@iconify/svelte';
</script>

<div class="navbar bg-base-300 rounded px-4">
	<div class="flex-1">
		<p>Students - Primary - A</p>
	</div>
	<div class="flex gap-2">
		<input
			type="text"
			placeholder="Search"
			class="input input-bordered w-38 transition-all ease-linear focus:w-64 focus:outline-none"
		/>

		<button
			class="btn btn-secondary"
			onclick={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" />
			FILTER
		</button>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-student-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Admit</button
		>
	</div>
</div>



<Filter />
<CreateStudent />
