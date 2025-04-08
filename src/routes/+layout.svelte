<script lang="ts">
	import Sidebar from '$lib/components/global/Sidebar.svelte';
	import Toast from '$lib/components/global/Toast.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<Sidebar>
	{@render children()}
</Sidebar>

<Toast />
