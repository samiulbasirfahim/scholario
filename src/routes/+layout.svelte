<script lang="ts">
	import Sidebar from '$lib/components/global/Sidebar.svelte';
	import Toast from '$lib/components/global/Toast.svelte';
	import { onMount } from 'svelte';
	import '../app.css';
	import { sessions } from '$lib/store/session.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { setting } from '$lib/store/setting.svelte';

	let { children } = $props();

	onMount(() => {
		sessions.fetch();
		sections.fetch();
        setting.fetch();
	});

	$effect(() => {
		if (sessions.selectedSession?.id) classes.fetch(sessions.selectedSession.id);
	});
</script>

<Sidebar>
	{@render children()}
</Sidebar>

<Toast />
