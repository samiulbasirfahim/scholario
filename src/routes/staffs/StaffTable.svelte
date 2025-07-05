<script lang="ts">
	let { staffs, selectedStaff = $bindable() } = $props();
</script>

<div class="flex w-full flex-1 flex-col overflow-hidden xl:w-1/2">
	<div class="bg-base-100 w-full overflow-scroll rounded">
		<div class="h-full overflow-auto">
			<table class="table-pin-rows table">
				<thead>
					<tr class="bg-base-200">
						<th>Name</th>
						<th>Role</th>
						<th>Qualification</th>
						<th>Salary</th>
					</tr>
				</thead>
				<tbody>
					{#each staffs as stf, i ((stf.id, i))}
						<tr
							class="{stf.id === selectedStaff
								? 'bg-primary text-primary-content'
								: ''} cursor-pointer"
							onclick={() => (selectedStaff = stf.id)}
						>
							<td>{stf.name}</td>
							<td>{stf.role}</td>
							<td>{stf.qualification || '-'}</td>
							<td>{stf.salary}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
