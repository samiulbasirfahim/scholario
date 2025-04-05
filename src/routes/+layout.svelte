<script lang="ts">
	import Sidebar from '$lib/components/global/Sidebar.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<Sidebar>
	{@render children()}
</Sidebar>
