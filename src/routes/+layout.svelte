<script lang="ts">
	import { afterNavigate, goto } from '$app/navigation';
	import Sidebar from '$lib/components/global/Sidebar.svelte';
	import Toast from '$lib/components/global/Toast.svelte';
	import { sessions } from '$lib/store/session.svelte';
	import { onMount } from 'svelte';
	import '../app.css';
	import { toast } from '$lib/store/toast.svelte';

	let { children } = $props();

	onMount(async () => {
		afterNavigate(() => {
			if (sessions.data.length === 0 && window.location.pathname !== '/manage') {
				goto('/manage');
				toast.set({
					message: 'No session found. Please create one to continue.',
					type: 'error'
				});
			}
		});
	});
</script>

<Sidebar>
	{@render children()}
</Sidebar>

<Toast />
