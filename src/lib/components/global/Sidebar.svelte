<script lang="ts">
	import Icon from '@iconify/svelte';
	import ThemeToggler from './ThemeToggler.svelte';
	import { page } from '$app/state';

	let { children } = $props();
</script>

{#snippet link(href: string, icon: string, title: string)}
	<a
		{href}
		class="flex items-center justify-start gap-4 p-2
		{page.url.pathname.split('/')[1] === href.split('/')[1]
			? 'bg-base-300 font-semibold'
			: 'font-light'}"
	>
		<Icon {icon} font-size="20" />
		{title}
	</a>
{/snippet}

<div class="drawer lg:drawer-open">
	<input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content flex flex-col items-center justify-center">
		<label for="my-drawer-2" class="fixed top-[50%] left-0 z-[999] cursor-grab lg:hidden">
			<Icon icon="memory:box-light-vertical-menu-right" font-size="30" />
		</label>
		{@render children()}
	</div>
	<div class="drawer-side">
		<label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
		<div class="bg-base-200 text-base-content flex min-h-full w-80 flex-col justify-between p-4">
			<ul class="menu w-full gap-2">
				<li>{@render link('/', 'material-symbols:dashboard-rounded', 'Dashboard')}</li>
				<li>{@render link('/admission', 'mingcute:user-add-fill', 'Admission')}</li>
				<li>{@render link('/students', 'fa6-solid:user-graduate', 'Students')}</li>
				<li>{@render link('/attendance', 'fa-solid:tasks', 'Attendance')}</li>
				<li>{@render link('/teachers', 'fa-solid:chalkboard-teacher', 'Teachers')}</li>
				<li>{@render link('/transaction', 'fa6-solid:sack-dollar', 'Transaction')}</li>
			</ul>
			<div class="flex items-center justify-between">
				<div class="dropdown-top dropdown dropdown-start">
					<div tabindex="0" role="button" class="btn m-1">
						<Icon icon="material-symbols:settings-rounded" font-size="20" />
					</div>
					<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
					<ul
						tabindex="0"
						class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm"
					>
						<li>
							<label for="my-modal" class="btn">open modal</label>
						</li>
						<li><a>Item 2</a></li>
					</ul>
				</div>
				<ThemeToggler />
			</div>
		</div>
	</div>
</div>
