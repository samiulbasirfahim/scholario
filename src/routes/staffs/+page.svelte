<script lang="ts">
	import Navbar from '$lib/components/global/Navbar.svelte';
	import { staff } from '$lib/store/staff.svelte';
	import type { Staff } from '$lib/types/staff';
	import Icon from '@iconify/svelte';

	import { goto } from '$app/navigation';
	import StaffTable from './StaffTable.svelte';
	import StaffDetails from './StaffDetails.svelte';
	import CreateStaff from '$lib/components/staff/CreateStaff.svelte';

	let selectedStaff = $state<number>();
	let selectedStaffData = $derived(staff.getById(selectedStaff ?? -1));

	$effect(() => {
		if (selectedStaff) {
			goto('?selectedStaff=' + selectedStaff, { replaceState: true });
		}
	});

	let filter = $state({
		role: '',
		qualification: ''
	});

	let isEditing = $state(false);

	let staffs = $state<Staff[]>([]);

	$effect(() => {
		(async () => {
			console.log(staff.reactiveCounter);
			staffs = await staff.getNonTeachers();
		})();
	});
</script>

<Navbar>
	<div class="flex-1">
		<div class="breadcrumbs text-sm font-semibold">
			<ul>
				<li>Staff</li>
				{#if filter.role}
					<li>{filter.role}</li>
				{/if}
				{#if filter.qualification}
					<li>{filter.qualification}</li>
				{/if}
			</ul>
		</div>
	</div>
	<div class="flex gap-2">
		<div class="isolate">
			<input
				type="text"
				placeholder="Search"
				class="input input-sm input-bordered w-48 transform-gpu transition-all ease-linear focus:w-64 focus:outline-none"
			/>
		</div>

		<button
			class="btn btn-secondary btn-sm"
			onclick={() => {
				(document.getElementById('filter-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="tabler:filter-filled" font-size="18" />
			FILTER
		</button>

		<button
			class="btn btn-primary btn-sm"
			onclick={() => {
				isEditing = false;
				(document.getElementById('create-staff-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" font-size="20" />
			Add Staff
		</button>
	</div>
</Navbar>

{#if staffs.length > 0}
	<div class="flex flex-1 gap-2 overflow-hidden">
		<StaffTable bind:selectedStaff {staffs} />
		<StaffDetails {selectedStaff} {selectedStaffData} bind:isEditing />
	</div>
{:else}
	<p class="text-secondary alert alert-warning text-sm">No staff found.</p>
{/if}

<CreateStaff bind:isEditing {selectedStaffData} />
