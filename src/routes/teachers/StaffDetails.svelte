<script lang="ts">
	import { sessions } from '$lib/store/session.svelte';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { toast } from '$lib/store/toast.svelte';
	import { staff } from '$lib/store/staff.svelte';

	let { selectedStaffData, selectedStaff, isEditing = $bindable() } = $props();

	const deleteStaff = async () => {
		if (!selectedStaff) return;
		try {
			await invoke('delete_staff', {
				id: selectedStaff,
				session_id: sessions.selected as number
			});
			toast.set({ message: 'Staff deleted', type: 'success' });
			selectedStaff = null;
		} catch (error) {
			console.error(error);
			toast.set({ message: 'Failed to delete staff', type: 'error' });
		}
	};
</script>

<div class="flex w-full flex-1 flex-col gap-0 overflow-hidden xl:w-1/2">
	<div class="bg-base-100 flex flex-col overflow-hidden rounded p-4">
		<div
			class="border-accent mb-3 flex items-center justify-between border-b-1 pb-2 text-xl font-bold"
		>
			<h1 class="text-primary">Staff Details</h1>
			<div class="join">
				<button
					class="btn btn-warning tooltip tooltip-bottom btn-sm join-item text-xl"
					data-tip="Edit"
					onclick={() => {
						isEditing = true;
						(document.getElementById('create-staff-modal') as HTMLDialogElement).show();
					}}
				>
					<Icon icon="uil:edit" font-size="22" />
				</button>

				<button
					class="btn btn-error tooltip tooltip-bottom btn-sm join-item text-xl"
					data-tip="Delete"
					onclick={deleteStaff}
				>
					<Icon icon="mdi:delete" />
				</button>
			</div>
		</div>

		{#if selectedStaff && selectedStaffData}
			<div class="text-sm">
				<div class="text-accent grid grid-cols-2 gap-2">
					<div class="space-y-2">
						<div>
							<p class="text-secondary">Name</p>
							<p class="font-medium">{selectedStaffData.name}</p>
						</div>

						<div>
							<p class="text-secondary">Role</p>
							<p class="font-medium">{selectedStaffData.role}</p>
						</div>

						<div>
							<p class="text-secondary">Qualification</p>
							<p class="font-medium">{selectedStaffData.qualification || 'N/A'}</p>
						</div>

						<div>
							<p class="text-secondary">Address</p>
							<p class="font-medium">{selectedStaffData.address || 'N/A'}</p>
						</div>

						<div>
							<p class="text-secondary">Phone</p>
							<p class="font-medium">{selectedStaffData.phone || 'N/A'}</p>
						</div>

						<div>
							<p class="text-secondary">Salary</p>
							<p class="font-medium">{selectedStaffData.salary}</p>
						</div>
					</div>

					<div class="space-y-2">
						<img
							src={selectedStaffData.photo}
							alt="Photo of {selectedStaffData.name}"
							class="h-40 w-40 flex-shrink-0 rounded object-cover"
						/>

						{#if selectedStaffData.health_note}
							<div>
								<p class="text-secondary text-xs">Health Notes</p>
								<div
									class="bg-base-200 overflow-y-auto rounded p-2"
									style="max-height: 80px; transform: translateZ(0);"
								>
									<p class="text-sm leading-snug font-medium break-words antialiased">
										{selectedStaffData.health_note}
									</p>
								</div>
							</div>
						{/if}

						{#if selectedStaffData.general_note}
							<div>
								<p class="text-secondary text-xs">General Notes</p>
								<div
									class="bg-base-200 overflow-y-auto rounded p-2"
									style="max-height: 80px; transform: translateZ(0);"
								>
									<p class="text-sm leading-snug font-medium break-words antialiased">
										{selectedStaffData.general_note}
									</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>
		{:else}
			<p class="text-secondary alert alert-warning text-sm">
				Select a staff member to view details.
			</p>
		{/if}
	</div>
</div>
