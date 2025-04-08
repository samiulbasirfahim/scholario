<script lang="ts">
	import { page } from '$app/state';
	const { children } = $props();

	const links = [
		['Dashboard', '/', 'lucide:layout-grid'],
		['Students', '/students', 'fa-solid:user-graduate'],
		['Classes', '/classes', 'fa-solid:user-graduate'],
		['Teachers', '/teachers', 'fa-solid:user-graduate'],
		['Staffs', '/staffs', 'fa-solid:user-graduate'],
		['Billing', '/billing', 'fa-solid:user-graduate'],
		['Exams', '/exams', 'fa-solid:user-graduate']
	];

	$effect(() => console.log(page.route.id));
</script>

<div class="drawer drawer-open">
	<input id="sidebar" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content space-y-4 p-4">
		{@render children()}
	</div>
	<div class="drawer-side">
		<label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
		<ul class="menu bg-base-300 text-base-content min-h-full w-80 p-4">
			<div class="flex flex-col items-center justify-center pt-4">
				<h1 class="text-primary text-center font-semibold">Holy Child International School</h1>
				<h1 class="text-center opacity-60">Dapunia, Mymensingh</h1>
			</div>
			<div class="divider"></div>
			<div class="">
				{#each links as link, i (i)}
					<li>
						<a
							class="btn flex items-center justify-start gap-2 p-2 {page.route.id === link[1]
								? 'btn-primary'
								: 'btn-ghost opacity-90 '}"
							href={link[1]}
						>
							{link[0]}
						</a>
					</li>
				{/each}
			</div>
		</ul>
	</div>
</div>
