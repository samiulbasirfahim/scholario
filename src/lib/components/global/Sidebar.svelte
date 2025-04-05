<script lang="ts">
	const { children } = $props();
	import Icon from '@iconify/svelte';

	const links = [
		['Dashboard', '/dashboard', 'lucide:layout-grid'],
		['Students', '/students', 'fa-solid:user-graduate'],
		['Classes', '/classes', 'fa-solid:user-graduate'],
		['Teachers', '/teachers', 'fa-solid:user-graduate'],
		['Staffs', '/staffs', 'fa-solid:user-graduate'],
		['Billing', '/billing', 'fa-solid:user-graduate'],
		['Exams', '/exams', 'fa-solid:user-graduate']
	];
</script>

<div class="drawer drawer-open">
	<input id="sidebar" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content flex flex-col items-center justify-center">
		{@render children()}
	</div>
	<div class="drawer-side">
		<label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
		<ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4">
			<div class="flex flex-col items-center justify-center pt-4">
				<div class="avatar mb-4">
					<div class="w-24 rounded-full">
						<img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" />
					</div>
				</div>
				<h1 class="text-primary text-center font-semibold">Holy Child International School</h1>
				<h1 class="text-center opacity-60">Dapunia, Mymensingh</h1>
			</div>
			<div class="divider"></div>
			<div class="space-y-2">
				{#each links as link, i ([link[0], i])}
					<li>
						<a class="flex items-center justify-start gap-2 p-2" href={link[1]}>
							<Icon icon={link[2]} />
							{link[0]}
						</a>
					</li>
				{/each}
			</div>
		</ul>
	</div>
</div>
