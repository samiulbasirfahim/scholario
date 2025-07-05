<script lang="ts">
	import { page } from '$app/state';
	import { setting } from '$lib/store/setting.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	const { children } = $props();

	const links = [
		['Dashboard', '/'],
		['Students', '/students'],
		['Attendance', '/attendance'],
		['Classes', '/classes'],
		['Teachers', '/teachers'],
		['Staffs', '/staffs'],
		['Billing', '/billing'],
		['Exams', '/exams'],
		['Session', '/session'],
		['Broadcast', '/broadcast']
	];

	let theme = $state<'LIGHT' | 'DARK'>('LIGHT');
	onMount(async function () {
		theme = await localStorage.theme;
		const htmlEl = document.documentElement;
		htmlEl.setAttribute('data-theme', theme === 'LIGHT' ? 'corporate-custom' : 'business-custom');
	});

	function toggleTheme() {
		const htmlEl = document.documentElement;
		theme = theme === 'LIGHT' ? 'DARK' : 'LIGHT';
		localStorage.theme = theme;
		htmlEl.setAttribute('data-theme', theme === 'LIGHT' ? 'corporate-custom' : 'business-custom');
	}
</script>

<div class="drawer drawer-open">
	<input id="sidebar" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content bg-base-300 box-border h-screen">
		<div class="flex h-full flex-col p-4">
			{@render children()}
		</div>
	</div>
	<div class="drawer-side">
		<label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
		<ul class="menu min-h-full w-80 justify-between">
			<div class="flex h-full flex-col justify-between">
				<div class="flex flex-col items-center justify-center pt-4">
					<h1 class="text-center text-base font-semibold">{setting.title}</h1>
					<h1 class="text-center opacity-90">{setting.address}</h1>
				</div>
				<div class="divider"></div>
				<ul class="space-y-2">
					{#each links as link, i (i)}
						<li>
							<a
								class="flex items-center justify-start gap-2 p-2 {page.url.pathname.split(
									'/'
								)[1] === link[1].split('/')[1]
									? 'bg-primary text-primary-content'
									: 'opacity-80'}"
								href={link[1]}
							>
								{link[0]}
							</a>
						</li>
					{/each}
				</ul>
			</div>
			<div class="text-accent hover:text-accent">
				<li class="border-accent flex flex-row justify-between border-b-1">
					<button
						class="btn btn-ghost tooltip-right tooltip-accent tooltip btn-square m-1 text-xl"
						data-tip="Backward"
						onclick={() => {
							window.history.back();
						}}
					>
						<Icon icon="mingcute:arrow-left-fill" />
					</button>
					<button
						class="btn btn-ghost tooltip tooltip-left tooltip-accent btn-square m-1 text-xl"
						data-tip="Forward"
						onclick={() => {
							window.history.forward();
						}}
					>
						<Icon icon="mingcute:arrow-right-fill" />
					</button>
				</li>
				<div class="flex justify-between">
					<button
						class="btn btn-square btn-ghost tooltip tooltip-accent tooltip-right m-1 text-xl"
						data-tip="toggle Theme"
						onclick={toggleTheme}
					>
						{#if theme === 'LIGHT'}
							<Icon icon="line-md:moon-filled-to-sunny-filled-transition" />
						{:else}
							<Icon icon="line-md:sunny-filled-loop-to-moon-filled-transition" />
						{/if}
					</button>
					<div class="dropdown dropdown-top dropdown-left">
						<div
							tabindex="0"
							role="button"
							class="btn btn-square btn-ghost tooltip tooltip-left tooltip-accent m-1 text-xl"
							data-tip="More"
						>
							<Icon icon="ep:more-filled" />
						</div>
						<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
						<ul
							tabindex="0"
							class="dropdown-content menu bg-base-100 rounded-box text-base-content z-1 w-52 p-2 shadow-sm"
						>
							<li>
								<a class="flex items-center justify-start gap-2 p-2" href="/broadcast">Broadcast</a>
							</li>
							<li>
								<a class="flex items-center justify-start gap-2 p-2" href="/manage">Manage</a>
							</li>

							<li>
								<a class="flex items-center justify-start gap-2 p-2" href="/test">Test</a>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</ul>
	</div>
</div>
